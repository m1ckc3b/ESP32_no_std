#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    dma::{Dma, DmaPriority, DmaRxBuf, DmaTxBuf},
    dma_buffers,
    prelude::*,
    spi::{
        master::{Config, Spi},
        SpiMode,
    },
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let sclk = peripherals.GPIO18;
    let miso = peripherals.GPIO19;
    let mosi = peripherals.GPIO23;
    let cs_1 = peripherals.GPIO5;
    // let cs_2 = peripherals.GPIO4;


    // Configure DMA
    // DMA driver and get a channel
    let dma = Dma::new(peripherals.DMA);
    let dma_channel = dma.spi2channel;

    // DMA transfers need descriptors and buffers
    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = dma_buffers!(32000);
    let mut dma_rx_buf = DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let mut dma_tx_buf = DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();


    // Configure SPI
    // we can call `.with_dma` on the SPI driver to make it use DMA
    let mut spi = Spi::new_with_config(
        peripherals.SPI2,
        Config {
            frequency: 100.kHz(),
            mode: SpiMode::Mode0,
            ..Config::default()
        },
    )
    .with_sck(sclk)
    .with_mosi(mosi)
    .with_miso(miso)
    .with_cs(cs_1)
    // .with_cs(cs_2)
    .with_dma(dma_channel.configure(false, DmaPriority::Priority0));


    let delay = Delay::new();

    // populate the tx_buffer with data to send
    dma_tx_buf.as_mut_slice().fill(1);

    loop {
        // Transfer
        // `dma_transfer` will move the driver and the buffers into the returned transfer.
        let transfer = spi
            .transfer(dma_rx_buf, dma_tx_buf)
            .map_err(|e| e.0)
            .unwrap();


        (spi, (dma_rx_buf, dma_tx_buf)) = transfer.wait();

        println!("Transfer done");
        
        println!("Sent {:x?} .. {:x?}", 
        &dma_tx_buf.as_slice()[..10], 
        &dma_tx_buf.as_slice().last_chunk::<10>().unwrap()
        );

        println!(
            "Received {:x?} .. {:x?}",
            &dma_rx_buf.as_slice()[..10],
            &dma_rx_buf.as_slice().last_chunk::<10>().unwrap()
        );

        delay.delay_millis(2500u32);
    }
}
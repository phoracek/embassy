#![no_std]
#![no_main]

use core::fmt::Write;
use core::str::from_utf8;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::spi::{Config, Spi};
use embassy_stm32::time::Hertz;
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut spi_config = Config::default();
    spi_config.frequency = Hertz(1_000_000);

    let mut spi = Spi::new(p.SPI1, p.PB3, p.PB5, p.PB4, p.DMA2_CH3, p.DMA2_CH2, spi_config);

    let mut read_buffer = [0; 128];
    let mut write_buffer = [0; 128];

    for n in 0u32.. {
        let mut write: String<128> = String::new();
        core::write!(&mut write, "Hello DMA World {}!\r\n", n).unwrap();
        let read_buffer = &mut read_buffer[..write.len()];
        let write_buffer = &mut write_buffer[..write.len()];
        write_buffer.clone_from_slice(write.as_bytes());

        spi.transfer(read_buffer, write_buffer).await.ok();
        info!("read via spi+dma: {}", from_utf8(read_buffer).unwrap());
    }
}

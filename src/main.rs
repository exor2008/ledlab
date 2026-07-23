#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;

use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Start");

    // Init pins
    let _ = embassy_rp::init(Default::default());

    loop {}
}

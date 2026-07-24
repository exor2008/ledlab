#![no_std]
#![no_main]

use embassy_rp::{
    pio::Instance,
    pio_programs::ws2812::{Grb, PioWs2812},
};

use crate::{
    buffer::Buffer,
    utils::Direction,
    world::{GetTicker, GetWorld, OnDirection, Switch, Tick},
};

pub mod apds9960;
pub mod buffer;
pub mod color;
pub mod cooldown;
pub mod perlin;
pub mod utils;
pub mod world;

async fn start<'d, Coord, B, W, P, const WORLDS: usize, const S: usize, const N: usize>(
    buffer: &mut B,
    switch: &mut Switch<WORLDS>,
    ws2812: &mut PioWs2812<'d, P, S, N, Grb>,
) where
    B: Buffer<Coord, N>,
    W: Tick<Coord, B, N> + GetTicker + GetWorld + OnDirection,
    P: Instance,
{
    let mut world = W::get_world(1);

    world.on_direction(Direction::Up);
    world = switch.switch_world();
    world.tick(buffer);
    ws2812.write(&buffer.data()).await;
    world.get_ticker().next().await;
}

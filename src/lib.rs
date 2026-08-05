#![no_std]
#![no_main]

use crate::{
    buffer::Buffer,
    world::{GetTicker, GetWorld, OnCommand, Switch, Tick},
};
use embassy_rp::{
    pio::Instance,
    pio_programs::ws2812::{Grb, PioWs2812},
};

pub mod buffer;
pub mod color;
pub mod cooldown;
pub mod perlin;
pub mod utils;
pub mod world;

pub trait CommandHandler<W, Coord, B, Command, const N: usize, const WORLDS: usize>
where
    W: Tick<Coord, B, N> + GetTicker + GetWorld + OnCommand<Command>,
    B: Buffer<Coord, N>,
{
    fn handle(&self, buffer: &mut B, world: &mut W, switch: &mut Switch<WORLDS>);
}

pub async fn start<
    'd,
    Coord,
    B,
    Command,
    W,
    H,
    P,
    const WORLDS: usize,
    const S: usize,
    const N: usize,
>(
    buffer: &mut B,
    switch: &mut Switch<WORLDS>,
    ws2812: &mut PioWs2812<'d, P, S, N, Grb>,
    handler: H,
) -> !
where
    B: Buffer<Coord, N>,
    W: Tick<Coord, B, N> + GetTicker + GetWorld + OnCommand<Command>,
    H: CommandHandler<W, Coord, B, Command, N, WORLDS>,
    P: Instance,
{
    let mut world = W::get_world(1);

    loop {
        handler.handle(buffer, &mut world, switch);

        world.tick(buffer);
        ws2812.write(&buffer.data()).await;
        world.get_ticker().next().await;
    }
}

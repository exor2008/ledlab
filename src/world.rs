use embassy_time::Ticker;

use crate::buffer::Buffer;

pub trait Tick<Coord, B, const N: usize>
where
    B: Buffer<Coord, N>,
{
    fn tick(&mut self, buffer: &mut B);
}

pub trait GetTicker {
    fn get_ticker(&mut self) -> &mut Ticker;
}

pub trait OnCommand<Command> {
    fn on_command(&mut self, command: Command);
}

pub trait GetWorld {
    fn get_world(index: usize) -> Self;
}

pub struct Switch<const WORLDS: usize> {
    counter: usize,
    is_on: bool,
}

impl<const WORLDS: usize> Default for Switch<WORLDS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const WORLDS: usize> Switch<WORLDS> {
    pub fn new() -> Self {
        Switch {
            counter: 1,
            is_on: true,
        }
    }

    pub fn switch_world<W: GetWorld>(&mut self) -> W {
        self.counter += 1;
        self.counter = if self.counter > WORLDS {
            1
        } else {
            self.counter
        };
        W::get_world(self.counter)
    }

    pub fn turn_off<W: GetWorld>(&mut self) -> W {
        W::get_world(0)
    }

    pub fn turn_on<W: GetWorld>(&mut self) -> W {
        W::get_world(self.counter)
    }

    pub fn switch_power<W: GetWorld>(&mut self) -> W {
        match self.is_on {
            true => {
                self.is_on = false;
                self.turn_off::<W>()
            }
            false => {
                self.is_on = true;
                self.turn_on::<W>()
            }
        }
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }
}

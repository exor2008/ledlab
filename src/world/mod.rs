use crate::{buffer::Buffer, utils::Direction};

pub trait Tick<Color, Coord, B>
where
    B: Buffer<Color, Coord>,
{
    type Ticker;

    fn tick(&mut self, buffer: &mut B);
    fn ticker(&mut self) -> &mut Self::Ticker;
    fn on_direction(&mut self, direction: Direction);
}

pub trait World {
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

    pub fn switch_world<W: World>(&mut self) -> W {
        self.counter += 1;
        self.counter = if self.counter > WORLDS {
            1
        } else {
            self.counter
        };
        W::get_world(self.counter)
    }

    pub fn turn_off<W: World>(&mut self) -> W {
        W::get_world(0)
    }

    pub fn turn_on<W: World>(&mut self) -> W {
        W::get_world(self.counter)
    }

    pub fn switch_power<W: World>(&mut self) -> W {
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
}

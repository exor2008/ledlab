#[derive(Debug, Clone, Copy)]
pub enum Command {
    Swing,
    SwitchPower,
    Level(Direction),
}

impl defmt::Format for Command {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            Command::Swing => defmt::write!(fmt, "Swing"),
            Command::SwitchPower => defmt::write!(fmt, "SwitchPower"),
            Command::Level(direction) => defmt::write!(fmt, "Level({:?})", direction),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
}

impl defmt::Format for Direction {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            Direction::Up => defmt::write!(fmt, "Up"),
            Direction::Down => defmt::write!(fmt, "Down"),
        }
    }
}

#[derive(Debug, Default)]
pub enum State {
    #[default]
    Check,
    Swing,
    Record,
}

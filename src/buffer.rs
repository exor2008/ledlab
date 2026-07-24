use smart_leds::RGB8;

pub trait Buffer<Coord, const N: usize> {
    fn write(&mut self, coord: Coord, color: RGB8);
    fn write_straight(&mut self, index: usize, color: RGB8);
    fn clear(&mut self);
    fn bg(&mut self, bg: RGB8);
    fn read(&self, coord: Coord) -> RGB8;
    fn data(&self) -> [RGB8; N];
}

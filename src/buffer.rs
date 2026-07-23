pub trait Buffer<Color, Coord> {
    fn write(&mut self, coord: Coord, color: Color);
    fn write_straight(&mut self, index: usize, color: Color);
    fn clear(&mut self);
    fn bg(&mut self, bg: Color);
    fn read(&self, coord: Coord) -> Color;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub fn swap( self, first_val: u8, second_val: u8) -> Color {
        let mut map = [self.r, self.g, self.b, self.a];
        for i in 0..4 {
            if map[i] == first_val {
                map[i] = second_val;
            } else if map[i] == second_val {
                map[i] = first_val;
            }
        }

        Color {
            r: map[0],
            g: map[1],
            b: map[2],
            a: map[3],
        }
    }}
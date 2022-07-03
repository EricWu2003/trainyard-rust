#[derive(Copy, Clone, Debug)]
pub struct Connection {
    pub dir1: u8,
    pub dir2: u8,
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        (self.dir1 == other.dir1 && self.dir2 == other.dir2)
            || (self.dir1 == other.dir2 && self.dir2 == other.dir1)
    }
}
impl Eq for Connection {}

impl Connection {
    pub fn rot(&self, angle: u8) -> Self {
        Connection {
            dir1: (self.dir1 + angle) % 4,
            dir2: (self.dir2 + angle) % 4,
        }
    }
    pub fn eq_up_to_rot(&self, other: &Self) -> bool {
        for rot_amt in 0..4 {
            if *other == self.rot(rot_amt) {
                return true;
            }
        }
        false
    }
    pub fn contains(&self, dir: u8) -> bool {
        self.dir1 == dir || self.dir2 == dir
    }
}

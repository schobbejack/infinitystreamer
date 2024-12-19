use std::fmt;

pub struct Dimension {
    width: u32,
    height: u32,
}

impl Dimension {
    pub fn new(width: u32, height: u32) -> Self {
        Dimension { width, height }
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!("{}x{}", self.width, self.height))
    }
}

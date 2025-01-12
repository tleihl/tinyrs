#[derive(Copy, Clone)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Resolution {
    pub fn new(pair: (u32, u32)) -> Self {
        Resolution{width: pair.0, height: pair.1}
    }
}

impl From<&Resolution> for Resolution {
    fn from(res: &Resolution) -> Self {
        res.clone()
    }
}

impl From<(u32, u32)> for Resolution {
    fn from(pair: (u32, u32)) -> Self {
        Resolution{width: pair.0, height: pair.1}
    }
}

impl Default for Resolution {
    fn default() -> Self {
        (1024, 768).into()
    }
}
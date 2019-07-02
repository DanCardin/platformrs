pub struct Config {
    pub tilesize: u16,
    pub screen_width: u32,
    pub screen_height: u32,
    pub scale: f32,
}

impl Config {
    pub fn new() -> Self {
        Self {
            tilesize: 70,
            screen_width: 70 * 18,
            screen_height: 70 * 15,
            scale: 1.0,
        }
    }
}

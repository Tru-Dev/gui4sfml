

use sfml::graphics::{Color, TextStyle, Font};
use sfml::system::SfBox;

use crate::resman::ResourceManager;

pub fn get_default_font() -> SfBox<Font> {
    Font::from_memory(
        include_bytes!("../assets/font/TitilliumWeb-Light.ttf")
    ).expect(&format!("Could not load default font!"))
}

/// The Theme trait. For implementations, look at the `themes` or
/// `macros` modules.
pub trait Theme {
    fn get_resman(&mut self) -> &mut ResourceManager;

    fn get_pallete_for(&self, widget: &str) -> [Color; 8] {
        [
            Color::rgb(159, 159, 159),
            Color::rgb(63, 63, 63),
            Color::rgb(79, 79, 79),
            Color::rgb(99, 99, 99),
            Color::rgb(99, 199, 0),
            Color::rgb(199, 99, 0),
            Color::rgb(199, 199, 0),
            Color::rgb(99, 99, 199),
        ]
    }

    fn get_pallete_color_for(&self, widget: &str, color: u8) -> Color {
        self.get_pallete_for(widget)[color as usize]
    }

    fn get_text_style_for(&self, widget: &str) -> TextStyle {
        TextStyle::REGULAR
    }

    fn get_font_for(&mut self, widget: &str) -> &SfBox<Font> {
        if !self.get_resman().has_font("default") {
            self.get_resman().set_from_font("default", &get_default_font());
        }
        self.get_resman().fntget("default")
    }
}

/// The default theme. Not much to look at.
pub struct DefaultTheme {
    resman: ResourceManager
}

impl DefaultTheme {
    pub fn new() -> DefaultTheme {
        DefaultTheme { resman: ResourceManager::new() }
    }
}

impl Theme for DefaultTheme {
    fn get_resman(&mut self) -> &mut ResourceManager {
        &mut self.resman
    }
}

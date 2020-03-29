use std::collections::BTreeMap;

use sfml::{
    // audio::{Sound, SoundBuffer},
    graphics::{
        Font, RenderTarget,
        RenderWindow, Text, Transformable, Texture, Sprite,
    },
    system::SfBox,
};

/// Caches `Texture`s to increase performance.
pub struct ResourceManager {
    textures: BTreeMap<String, SfBox<Texture>>,
    fonts: BTreeMap<String, SfBox<Font>>,
}

impl ResourceManager {
    /// Creates a new `ResourceManager`.
    pub fn new() -> ResourceManager {
        ResourceManager { textures: BTreeMap::new(), fonts: BTreeMap::new() }
    }

    /// Gets a `Texture` by name, loading from
    /// filename if it hasn't already been loaded.
    pub fn texget(&mut self, name: &str) -> &mut SfBox<Texture> {
        self.textures.entry(name.to_owned()).or_insert_with(|| {
            Texture::from_file(name)
                .expect(&format!("Unable to load texture: {}", name))
        })
    }

    /// Gets a `Texture` by name using the `get`
    /// method, and makes a sprite from it.
    pub fn texget_as_sprite(&mut self, name: &str) -> Sprite {
        Sprite::with_texture(self.texget(name))
    }

    pub fn set_from_texture(&mut self, name: &str, texture: &Texture) {
        self.textures.insert(name.to_owned(), texture.to_owned());
    }

    pub fn fntget(&mut self, name: &str) -> &SfBox<Font> {
        self.fonts.entry(name.to_owned()).or_insert_with(|| {
            Font::from_file(name)
                .expect(&format!("Unable to load font: {}", name))
        })
    }

    pub fn set_from_font(&mut self, name: &str, font: &Font) {
        self.fonts.insert(name.to_owned(), font.to_owned());
    }

    pub fn has_font(&self, name: &str) -> bool {
        if let None = self.fonts.get(name) {
            false
        } else {
            true
        }
    }
}

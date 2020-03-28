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
pub struct TextureManager {
    textures: BTreeMap<String, SfBox<Texture>>,
}

impl TextureManager {
    /// Creates a new `TextureManager`.
    pub fn new() -> TextureManager {
        TextureManager { textures: BTreeMap::new() }
    }

    /// Gets a `Texture` by name, loading from
    /// filename if it hasn't already been loaded.
    pub fn get(&mut self, name: &str) -> &mut SfBox<Texture> {
        self.textures.entry(name.to_owned()).or_insert_with(|| {
            Texture::from_file(name)
                .expect(&format!("Unable to load texture: {}", name))
        })
    }

    /// Gets a `Texture` by name using the `get`
    /// method, and makes a sprite from it.
    pub fn get_as_sprite(&mut self, name: &str) -> Sprite {
        Sprite::with_texture(self.get(name))
    }

    pub fn set_from_texture(&mut self, name: &str, texture: &Texture) {
        self.textures.insert(name.to_owned(), texture.to_owned());
        println!("{:?}", self.textures.get(name));
    }
}

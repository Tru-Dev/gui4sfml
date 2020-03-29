
use sfml::graphics::{
    Text, Font, Color, RenderStates, RenderTarget, RectangleShape, Shape
};
use sfml::system::{Vector2f, SfBox};
use sfml::window::Event;

#[allow(unused_imports)]
#[macro_use]
use crate::macros;
use crate::core::{WidgetTrait};
use crate::theming::{Theme, self};

/// A simple text label that can have a background.
pub struct Label<'f> {
    bg: Color,
    pub text: Text<'f>,
}

impl<'f> Label<'f> {
    pub fn new(text: &str, size: u32) -> Label<'f> {
        let mut t = Text::default();
        t.set_string(text);
        t.set_character_size(size);
        Label {
            bg: Color::TRANSPARENT,
            text: t,
        }
    }
}

impl<'f> WidgetTrait<'f> for Label<'f> {
    fn wdraw<'a: 's, 't, 's, 's_t>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: RenderStates<'t, 's, 's_t>
    ) {
        let lb = self.text.local_bounds();
        let mut r = RectangleShape::with_size((lb.width, lb.height).into());
        r.set_fill_color(self.bg);
        target.draw_with_renderstates(&r, states);
        target.draw_with_renderstates(&self.text, states);
    }

    fn wbind(&mut self, theme: &'f mut dyn Theme) {
        self.text.set_font(theme.get_font_for("Label"));
        self.text.set_fill_color(
            // theme.get_pallete_color_for("Label", pallete_map!(fg))
            Color::WHITE
        );
        self.bg = 
            // theme.get_pallete_color_for("Label", pallete_map!(bg));
            Color::TRANSPARENT;
    }

    fn process_event(&mut self, rel_pos: Vector2f, event: &Event) -> bool
    {false}

    fn set_theme(&mut self, theme: &'f mut dyn Theme) {

    }
}


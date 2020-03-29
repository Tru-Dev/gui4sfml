
use sfml::graphics::{
    RenderWindow, Color, RenderTarget, VertexArray, PrimitiveType, Vertex,
    Text, RenderStates
};
use sfml::window::{Style, Event, Key};

use ::gui4sfml::{Root, Widget, DefaultTheme, Label, };
use ::gui4sfml::theming::Theme;
use ::gui4sfml::core::WidgetTrait;

fn main() {
    let mut root = Root::new();
    let mut theme = DefaultTheme::new();

    let l = Widget::new(Label::new("Hello!", 80), (0., 0.), &mut theme);

    root.add_widget(l);

    let mut rwin = RenderWindow::new(
        (800, 600), "gui4sfml", Style::TITLEBAR, &Default::default()
    );
    rwin.set_position((0, 0).into());
    rwin.set_vertical_sync_enabled(true);

    let mut varr = VertexArray::new(PrimitiveType::TriangleStrip, 4);
    varr.append(&Vertex::with_pos_color((0., 0.), Color::BLUE));
    varr.append(&Vertex::with_pos_color((100., 0.), Color::CYAN));
    varr.append(&Vertex::with_pos_color((0., 100.), Color::BLUE));
    varr.append(&Vertex::with_pos_color((100., 100.), Color::CYAN));

    loop {
        while let Some(event) = rwin.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                _ => {}
            }
        }

        rwin.clear(Color::BLACK);
        rwin.draw(&root);
        // rwin.draw(&txt);
        // rwin.draw(&varr);

        rwin.display();
    }
}

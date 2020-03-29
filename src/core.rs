use std::cell::RefCell;
use std::rc::Rc;
use std::f32::consts;

use sfml::{
    // audio::{Sound, SoundBuffer},
    graphics::{
        Font, RenderTarget, Drawable, FloatRect, RenderTexture,
        Text, Transformable, Texture, Sprite, RenderStates, Transform
    },
    system::{Vector2f},
    window::{Event, Key, Style},
};

use crate::theming::Theme;

/// UI's root. All `Widget`s are stored here.
pub struct Root {
    widgets: Vec<Widget>,
}

impl Root {
    pub fn new() -> Root {
        let widgets = Vec::new();
        Root { widgets }
    }

    pub fn add_widget(&mut self, w: Widget) {
        self.widgets.push(w);
    }
}

impl Drawable for Root {
    fn draw<'a: 's, 't, 's, 's_t>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: RenderStates<'t, 's, 's_t>
    ) {
        for w in &self.widgets {
            target.draw_with_renderstates(w, states)
        }
    }
}

pub trait WidgetTrait {
    fn wdraw<'a: 's, 't, 's, 's_t>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: RenderStates<'t, 's, 's_t>,
    );
    fn wbind(&mut self, theme: &mut dyn Theme);
    fn process_event(&mut self, rel_pos: Vector2f, event: &Event) -> bool;
    fn set_theme(&mut self, theme: &mut dyn Theme);
}

#[derive(Clone, Copy)]
pub struct TransformRect {
    pub rect: FloatRect,
    pub t_origin: Vector2f,
    pub scaling: Vector2f,
    pub angle: f32,
}

impl TransformRect {
    pub fn from_rect(rect: FloatRect) -> TransformRect {
        TransformRect {
            rect,
            t_origin: (0., 0.).into(),
            scaling: (0., 0.).into(),
            angle: 0.
        }
    }

    pub fn from_point(x: f32, y: f32) -> TransformRect {
        TransformRect {
            rect: FloatRect::new(x, y, 0., 0.),
            t_origin: (0., 0.).into(),
            scaling: (0., 0.).into(),
            angle: 0.
        }
    }
}

impl Transformable for TransformRect {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        let v = position.into();

        self.rect.left = v.x;
        self.rect.top = v.y;
    }

    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        self.scaling = scale.into();
    }

    fn set_rotation(&mut self, angle: f32) {
        self.angle = angle;
    }

    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        self.t_origin = origin.into();
    }

    fn position(&self) -> Vector2f {
        (self.rect.left, self.rect.top).into()
    }

    fn rotation(&self) -> f32 {
        self.angle
    }

    fn get_scale(&self) -> Vector2f {
        self.scaling
    }

    fn origin(&self) -> Vector2f {
        self.t_origin
    }

    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        let v = offset.into();

        self.rect.left += v.x;
        self.rect.top += v.y;
    }

    fn rotate(&mut self, angle: f32) {
        self.angle += angle;
    }

    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        let v = factors.into();

        self.scaling.x *= v.x;
        self.scaling.y *= v.y;
    }

    fn transform(&self) -> Transform {
        let rads = -self.angle * consts::PI / 180.;
        let cos = rads.cos();
        let sin = rads.sin();
        let sxc = self.scaling.x * cos;
        let syc = self.scaling.y * cos;
        let sxs = self.scaling.x * sin;
        let sys = self.scaling.y * sin;
        let tx =
            -self.t_origin.x * sxc - self.t_origin.y * sys + self.rect.left;
        let ty =
            self.t_origin.x * sxs - self.t_origin.y * syc + self.rect.top;

        Transform::new(
            sxc,  sys, tx,
            -sxs, syc, ty,
            0.,   0.,  1.
        )
    }

    fn inverse_transform(&self) -> Transform {
        self.transform().inverse()
    }
}

impl From<(f32, f32)> for TransformRect {
    fn from(p: (f32, f32)) -> Self {
        TransformRect::from_point(p.0, p.1)
    }
}

impl From<(f32, f32, f32, f32)> for TransformRect {
    fn from(p: (f32, f32, f32, f32)) -> Self {
        TransformRect::from_rect(FloatRect::new(p.0, p.1, p.2, p.3))
    }
}

impl From<((f32, f32), (f32, f32))> for TransformRect {
    fn from(p: ((f32, f32), (f32, f32))) -> Self {
        TransformRect::from_rect(FloatRect::from_vecs(p.0.into(), p.1.into()))
    }
}

#[derive(Clone)]
pub struct Widget {
    pub widget: Rc<RefCell<dyn WidgetTrait>>,
    pub tsf_rc: TransformRect,
}

impl Widget {
    pub fn new<W: WidgetTrait + 'static, R: Into<TransformRect>>(
        mut widget: W,
        tsf_rc: R,
        theme: &mut dyn Theme
    ) -> Widget
    {
        widget.wbind(theme);
        Widget {
            widget: Rc::new(RefCell::new(widget)), tsf_rc: tsf_rc.into()
        }
    }
}

impl Drawable for Widget {
    fn draw<'a: 's, 't, 's, 's_t>(
        &'a self,
        target: &mut dyn RenderTarget,
        mut states: RenderStates<'t, 's, 's_t>
    ) {
        states.transform = self.tsf_rc.transform();
        self.widget.borrow().wdraw(target, states);
    }
}

pub trait ContainerTrait: WidgetTrait {
    fn add_widget(&mut self, widget: Widget);
    fn get_children(&self) -> &Vec<Widget>;
    fn get_children_mut(&mut self) -> &mut Vec<Widget>;
    fn draw_children<'a: 's, 't, 's, 's_t>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: RenderStates<'t, 's, 's_t>
    ) {
        for child in self.get_children() {
            child.draw(target, states);
        }
    }
}

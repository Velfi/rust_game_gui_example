use ggez::graphics;
use ggez::graphics::Point2;

pub type WidgetId = usize;

/**
 * A trait for widgets whose bounds may contain a given x,y coordinate
 */
pub trait IntersectsPoint {
    // Screen Origin is upper left in ggez
    fn intersects_point(&self, x: f32, y: f32) -> bool;
}

pub trait Rectangle {
    fn get_pos_x(&self) -> f32;
    fn get_width(&self) -> f32;
    fn get_pos_y(&self) -> f32;
    fn get_height(&self) -> f32;
}

pub fn rect_into_points(rect: impl Rectangle) -> [Point2; 4] {
    [
        Point2::new(rect.get_pos_x(), rect.get_pos_y()),
        Point2::new(rect.get_pos_x() + rect.get_width(), rect.get_pos_y()),
        Point2::new(
            rect.get_pos_x() + rect.get_width(),
            rect.get_pos_y() + rect.get_height(),
        ),
        Point2::new(rect.get_pos_x(), rect.get_pos_y() + rect.get_height()),
    ]
}

impl Rectangle for graphics::Rect {
    fn get_pos_x(&self) -> f32 {
        self.x
    }
    fn get_width(&self) -> f32 {
        self.w
    }
    fn get_pos_y(&self) -> f32 {
        self.y
    }
    fn get_height(&self) -> f32 {
        self.h
    }
}

pub trait GetMesh<T> {
    fn get_mesh(&self) -> T;
}

impl<R> IntersectsPoint for R
where
    R: Rectangle,
{
    fn intersects_point(&self, x: f32, y: f32) -> bool {
        x >= self.get_pos_x()
            && x <= self.get_pos_x() + self.get_width()
            && y >= self.get_pos_y()
            && y <= self.get_pos_y() + self.get_height()
    }
}

impl<R> GetMesh<graphics::Rect> for R
where
    R: Rectangle,
{
    fn get_mesh(&self) -> graphics::Rect {
        graphics::Rect {
            x: self.get_pos_x(),
            y: self.get_pos_y(),
            w: self.get_width(),
            h: self.get_height(),
        }
    }
}

#[derive(Debug)]
pub enum WidgetEvent {
    Press(WidgetId),
    Release(WidgetId),
    Enter(WidgetId),
    Leave(WidgetId),
}

#[derive(Debug, Clone, Copy)]
struct WidgetPrimitive {
    height: f32,
    position_x: f32,
    position_y: f32,
    width: f32,
}

impl Default for WidgetPrimitive {
    fn default() -> Self {
        WidgetPrimitive {
            height: 0.0,
            position_x: 0.0,
            position_y: 0.0,
            width: 0.0,
        }
    }
}

impl Rectangle for WidgetPrimitive {
    fn get_pos_x(&self) -> f32 {
        self.position_x
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn get_pos_y(&self) -> f32 {
        self.position_y
    }

    fn get_height(&self) -> f32 {
        self.height
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Button {
    inner_widget: WidgetPrimitive,
    //    is_clicked: bool,
    //    is_hovered: bool,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            inner_widget: Default::default(),
        }
    }
}

impl Button {
    pub fn set_position(&mut self, position_x: f32, position_y: f32) {
        self.inner_widget.position_x = position_x;
        self.inner_widget.position_y = position_y;
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.inner_widget.height = height;
        self.inner_widget.width = width;
    }
}

impl IntersectsPoint for Button {
    fn intersects_point(&self, x: f32, y: f32) -> bool {
        self.inner_widget.intersects_point(x, y)
    }
}

impl GetMesh<graphics::Rect> for Button {
    fn get_mesh(&self) -> graphics::Rect {
        self.inner_widget.get_mesh()
    }
}

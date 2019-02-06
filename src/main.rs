mod ui_layer;
mod widgets;

use ggez::{
    conf, event, event::MouseButton, event::MouseState, graphics, graphics::Drawable, Context,
    GameResult,
};
use ui_layer::UiLayer;
use widgets::Button;

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}

struct MainState {
    ui: UiLayer,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut ui = UiLayer::default();
        ui.get_stream()
            .observe(|widget_event| println!("{:?}", widget_event.as_ref()));

        let mut button = Button::default();
        button.set_size(120.0, 60.0);
        button.set_position(100.0, 50.0);

        let mut button2 = Button::default();
        button2.set_size(120.0, 60.0);
        button2.set_position(75.0, 300.0);

        let mut button3 = Button::default();
        button3.set_size(120.0, 60.0);
        button3.set_position(400.0, 200.0);

        ui.button_widgets.push(button);
        ui.button_widgets.push(button2);
        ui.button_widgets.push(button3);

        let s = MainState { ui };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let ui_mesh = self.ui.build_draw_mesh().build(ctx)?;
        ui_mesh.draw(ctx, graphics::Point2::new(0.0, 0.0), 0.0)?;
        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: i32,
        y: i32,
    ) {
        self.ui.check_press_and_emit_events(x as f32, y as f32);
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _state: MouseState,
        x: i32,
        y: i32,
        _xrel: i32,
        _yrel: i32,
    ) {
        self.ui
            .update_mouse_position_and_emit_events(x as f32, y as f32);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: i32, y: i32) {
        self.ui.check_release_and_emit_events(x as f32, y as f32);
    }
}

//struct Store {
//    counter: i32,
//}
//
//impl Store {
//    pub fn new() -> Self {
//        Self { counter: 0 }
//    }
//
//    pub fn reduce(&mut self, option_action: Option<Action>) {
//        match option_action {
//            Some(action) => match action {
//                Action::Increment => self.counter += 1,
//                Action::Decrement => self.counter -= 1,
//            },
//            _ => (),
//        }
//    }
//}
//
//enum Action {
//    Increment,
//    Decrement,
//}
//
//struct App {
//    store: Store,
//}
//
//impl App {
//    pub fn new() -> Self {
//        Self {
//            store: Store::new(),
//        }
//    }
//
//    pub fn store(&self) -> &Store {
//        &self.store
//    }
//
//    pub fn dispatch(&mut self, action: Action) {
//        self.store.reduce(Some(action));
//    }
//}

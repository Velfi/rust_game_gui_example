use crate::widgets::{self, rect_into_points, GetMesh, IntersectsPoint, WidgetEvent, WidgetId};
use frappe::{Sink, Stream};
use ggez::graphics;

const AVERAGE_WIDGET_AMOUNT: usize = 8;

pub struct UiLayer {
    pub button_widgets: Vec<widgets::Button>,
    event_sink: Sink<WidgetEvent>,
    new_hover_state: Vec<bool>,
    previous_hover_state: Vec<bool>,
    //    needs_update: bool,
    //    cached_mesh_builder: graphics::MeshBuilder,
}

impl Default for UiLayer {
    fn default() -> UiLayer {
        UiLayer {
            button_widgets: Vec::with_capacity(AVERAGE_WIDGET_AMOUNT),
            event_sink: Sink::new(),
            new_hover_state: Vec::with_capacity(AVERAGE_WIDGET_AMOUNT),
            previous_hover_state: Vec::with_capacity(AVERAGE_WIDGET_AMOUNT),
        }
    }
}

impl UiLayer {
    pub fn build_draw_mesh(&self) -> graphics::MeshBuilder {
        let mut mb = graphics::MeshBuilder::new();
        for button_widget in self.button_widgets.iter() {
            let rect = rect_into_points(button_widget.get_mesh());
            mb.polygon(graphics::DrawMode::Fill, &rect);
        }

        mb
    }

    // This is fine so long as stuff never gets removed. Currently, that could cause weird hover states
    // because we're just using index as an id.
    pub fn update_mouse_position_and_emit_events(&mut self, x: f32, y: f32) {
        self.previous_hover_state.clear();
        self.previous_hover_state.append(&mut self.new_hover_state);

        for button_widget in self.button_widgets.iter() {
            self.new_hover_state
                .push(button_widget.intersects_point(x, y));
        }

        for (widget_id, (previous_state, new_state)) in self
            .previous_hover_state
            .iter()
            .zip(self.new_hover_state.iter())
            .enumerate()
        {
            self.check_hover_state_change_and_emit_events(widget_id, *previous_state, *new_state);
        }
    }

    pub fn check_press_and_emit_events(&self, x: f32, y: f32) {
        for (index, button_widget) in self.button_widgets.iter().enumerate() {
            if button_widget.intersects_point(x, y) {
                self.event_sink.send(WidgetEvent::Press(index));
            }
        }
    }

    pub fn check_release_and_emit_events(&self, x: f32, y: f32) {
        for (index, button_widget) in self.button_widgets.iter().enumerate() {
            if button_widget.intersects_point(x, y) {
                self.event_sink.send(WidgetEvent::Release(index));
            }
        }
    }

    pub fn get_stream(&self) -> Stream<WidgetEvent> {
        self.event_sink.stream()
    }

    fn check_hover_state_change_and_emit_events(
        &self,
        widget_id: WidgetId,
        prev_state: bool,
        new_state: bool,
    ) {
        match (prev_state, new_state) {
            (false, true) => self.event_sink.send(WidgetEvent::Enter(widget_id)),
            (true, false) => self.event_sink.send(WidgetEvent::Leave(widget_id)),
            _ => (),
        }
    }
}

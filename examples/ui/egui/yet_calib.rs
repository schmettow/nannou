use nannou::prelude::*;
use nannou_egui::{egui, Egui};

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 360.0;
const MAX_TARGETS: u8 = 16;

enum State {Init, Welcome, Calibration}
type CalPoint = (i16, i16);

enum CalibOrder {Index, MinDistance}
struct Calibration {
    pub points: Vec<CalPoint>,
}

/*struct CalIterator<'a> {
    points: &'a Calibration,
    index: usize,
}*/
/*impl<'a> Iterator for CalIterator<'a> {
    type Item = &'a CalPoint;

    fn next(&mut self) -> Option<Self::CalPoint> {
        if self.index < self.points.list.len() {
            let result = Some(&self.todos.list[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}*/


pub struct CalIntoIterator {
    calib: Calibration
}

impl IntoIterator for Calibration {
    type Item = CalPoint;
    type IntoIter = CalIntoIterator;

    fn into_iter(self) -> CalIntoIterator {
        let cal_iter = CalIntoIterator { calib: self };
    }
}



fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    egui: Egui,
    radius: f32,
    color: Hsv,
}

fn model(app: &App) -> Model {
    // Create a new window! Store the ID so we can refer to it later.
    let window_id = app
        .new_window()
        .title("Nannou + Egui")
        .size(WIDTH as u32, HEIGHT as u32)
        .raw_event(raw_window_event) // This is where we forward all raw events for egui to process them
        .view(view) // The function that will be called for presenting graphics to a frame.
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();

    Model {
        egui: Egui::from_window(&window),
        radius: 40.0,
        color: hsv(10.0, 0.5, 1.0),
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let Model {
        ref mut egui,
        ref mut radius,
        ref mut color,
    } = *model;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings")
        .default_size(egui::vec2(0.0, 200.0))
        .show(&ctx, |ui| {
            ui.separator();
            ui.label("Number of targets per col/row");
            ui.add(egui::Slider::new(radius, 4..=MAX_TARGETS).text("Targets"));
            edit_hsv(ui, color);
        });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    frame.clear(BLACK);


    draw.ellipse()
        .x_y(100.0, 100.0)
        .radius(model.radius)
        .color(model.color);

    draw.to_frame(app, &frame).unwrap();

    // Do this as the last operation on your frame.
    model.egui.draw_to_frame(&frame).unwrap();
}

fn edit_hsv(ui: &mut egui::Ui, color: &mut Hsv) {
    let mut egui_hsv = egui::ecolor::Hsva::new(
        color.hue.to_positive_radians() as f32 / (std::f32::consts::PI * 2.0),
        color.saturation,
        color.value,
        1.0,
    );

    if egui::color_picker::color_edit_button_hsva(
        ui,
        &mut egui_hsv,
        egui::color_picker::Alpha::Opaque,
    )
    .changed()
    {
        *color = nannou::color::hsv(egui_hsv.h, egui_hsv.s, egui_hsv.v);
    }
}

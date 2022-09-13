extern crate nannou;
mod geometry_functions;
use nannou::prelude::*;
use nannou::prelude::geom::Vec2;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn get_wiggler_tail(width: i32, wiggle_rate: f32, wiggle_intensity: f32, tail_length: usize, time: f32) -> Vec<Point2> {
    let twirlAngle = time * TAU * wiggle_rate;
    let twirl = twirlAngle.sin() * PI / 2.0;

    let mut side1 : Vec<Point2> = Vec::with_capacity(tail_length);
    let mut side2 : Vec<Point2> = Vec::with_capacity(tail_length);

    for i in 1..tail_length {
        let position = i as f32 / tail_length as f32;
        let position_width = width as f32 * (position * PI / 2.0).cos().pow(4);
        let theta = position * &wiggle_intensity * twirl;
        let angle = (PI / 2.0) + theta;
        let x1 = angle.cos() * i as f32;
        let y1 = angle.sin() * i as f32;
        let p1 = pt2(x1, y1);

        let x2 = (angle + PI/2.0).cos() * position_width / 2.0 as f32;
        let y2 = (angle + PI/2.0).sin()* position_width / 2.0 as f32;
        let p2 = pt2(x2, y2) + p1;
        side1.push(p2);

        let x3 = (angle - PI/2.0).cos() * position_width / 2.0 as f32;
        let y3 = (angle - PI/2.0).sin() * position_width / 2.0 as f32;
        let p3 = pt2(x3, y3) + p1;
        side2.push(p3);
    }

    side1.append(&mut side2);
    side1
}

fn view(_app: &App, _model: &Model, frame: Frame){
    frame.clear(PINK);

    let draw = _app.draw();

    let circle_start = pt2(-50.0, 0.0);
    let circle_end = pt2(50.0, 0.0);
    let mut semi_circle_points = geometry_functions::semi_circle(circle_end, circle_start);

    // draw.ellipse().width(100.0).height(100.0);

    let mut points = get_wiggler_tail(100, 1.5, 0.5, 300, _app.time);
    points.append(&mut semi_circle_points);

    draw.polyline()
        .weight(3.0)
        .points(points);

    draw.to_frame(_app, &frame).unwrap();
}

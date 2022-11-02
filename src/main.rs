extern crate nannou;
mod geometry_functions;
use nannou::prelude::*;

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

fn get_wiggler_tail(width: i32, wiggle_rate: f32, wiggle_intensity: f32, tail_length: usize, time: &f32, x_offset: &f32, y_offset: &f32) -> Vec<Point2> {
    let twirl_angle = time * TAU * wiggle_rate;
    let twirl = twirl_angle.sin() * PI / 2.0;

    let mut side1 : Vec<Point2> = Vec::with_capacity(tail_length);
    let mut side2 : Vec<Point2> = Vec::with_capacity(tail_length.clone());

    for i in 1..tail_length {
        let position = i as f32 / tail_length.clone() as f32;
        let position_width = width.clone() as f32 * (position * PI / 2.0).cos().pow(4);
        let theta = position * &wiggle_intensity * twirl;
        let angle = (PI / 2.0) + theta;
        let x1 = angle.cos() * i as f32;
        let y1 = angle.sin() * i as f32;
        let p1 = pt2(x1, y1);

        let x2 = (angle + PI/2.0).cos() * position_width / 2.0 as f32;
        let y2 = (angle + PI/2.0).sin()* position_width / 2.0 as f32;
        let p2 = pt2(x2 + x_offset, y2 + y_offset) + p1;
        side1.push(p2);

        let x3 = (angle - PI/2.0).cos() * position_width / 2.0 as f32;
        let y3 = (angle - PI/2.0).sin() * position_width / 2.0 as f32;
        let p3 = pt2(x3 + x_offset, y3 + y_offset) + p1.clone();
        side2.push(p3);
    }

    let mut reversed_side_1 = Vec::with_capacity(tail_length.clone());
    for i in 0..side1.len() {
        reversed_side_1.push(side1[side1.len() - 1 - i])
    }
    side2.append(&mut reversed_side_1);
    side2
}

fn draw_wiggler(draw: &Draw, width: i32, wiggle_rate: f32, wiggle_intensity: f32, tail_length: usize, time: &f32, x_offset: &f32, y_offset: &f32){
    let circle_start = pt2(width.clone() as f32 / -2.0 + x_offset, 0.0 + y_offset);
    let circle_end = pt2(width.clone() as f32 / 2.0 + x_offset, 0.0 + y_offset);
    let mut semi_circle_points = geometry_functions::semi_circle(circle_end, circle_start);

    // draw.ellipse().width(100.0).height(100.0);

    let mut wiggler_tail = get_wiggler_tail(width, wiggle_rate, wiggle_intensity, tail_length, time, x_offset, y_offset);
    let mut points = Vec::with_capacity((&width * 2 + 180 + 1).try_into().unwrap());
    let meeting_point = pt2(wiggler_tail[0].x, wiggler_tail[0].y);
    points.append(&mut wiggler_tail);
    points.append(&mut semi_circle_points);
    points.push(meeting_point);

    let circle_start_2 = pt2(width.clone() as f32 / -4.0 + x_offset, 0.0 + y_offset);
    let circle_end_2 = pt2(width.clone() as f32 / 4.0 + x_offset, 0.0 + y_offset);
    let mut semi_circle_points_2 = geometry_functions::semi_circle(circle_end_2, circle_start_2);

    let mut wiggler_tail_2 = get_wiggler_tail(width.clone() / 2, wiggle_rate.clone(), wiggle_intensity.clone(), tail_length.clone(), time, x_offset, y_offset);
    let mut points_2 = Vec::with_capacity((&width + 180 + 1).try_into().unwrap());
    let meeting_point_2 = pt2(wiggler_tail_2[0].x, wiggler_tail_2[0].y);
    points_2.append(&mut wiggler_tail_2);
    points_2.append(&mut semi_circle_points_2);
    points_2.push(meeting_point_2);

    draw.polygon()
        .points(points)
        .color(PURPLE);
    draw.polygon()
        .points(points_2)
        .color(LIMEGREEN);
}

fn view(_app: &App, _model: &Model, frame: Frame){
    frame.clear(PINK);

    let draw = _app.draw();
    let x_offset: f32 = 200.0;
    let y_offset: f32 = 200.0;
    draw_wiggler(&draw,150, 1.5, 0.5, 200, &_app.time, &x_offset, &y_offset);

    let x_offset: f32 = -200.0;
    let y_offset: f32 = -200.0;
    draw_wiggler(&draw,120, 1.1, 0.8, 300, &_app.time, &x_offset, &y_offset);
    draw.to_frame(_app, &frame).unwrap();
}

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

/*
    @param width the width of the wiggler at the widest point
    @param wiggle_rate the speed that the wiggler completes one full arc in seconds
    @param wiggle_intensity the maximum angle that the tail sweeps around to
    @param time the time variable of the frame in which it is drawmn
    @param tail_length the length of the tail from the centre of the semi circle head
    @param x_offset the amount to offset all points by in the x axis
    @param y_offset the amount to offset all points by in the y axis
    @returns an array of points of a completed wiggler tail
 */
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

fn get_wiggler_points(draw: &Draw, width: i32, wiggle_rate: f32, wiggle_intensity: f32, tail_length: usize, time: &f32, x_offset: &f32, y_offset: &f32) -> Vec<Point2>{
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

    points
}

fn view(_app: &App, _model: &Model, frame: Frame){
    let wiggler_rows = 3;
    let wiggler_columns = 8;
    let hue = _app.time.clone() / 20.0;  //(6_app.time.clone() as i32 % 30) as f32;
    // println!("hue: {}", hue);
    // let hue = _app.time % 360;
    let background_color = TURQUOISE;// hsl (hue, 3.0, 20.0);
    frame.clear(background_color);

    let win = _app.window_rect();
    let max_x = win.x.end;
    let min_x = win.x.start;
    let min_y = win.y.start;
    let max_y = win.y.end;
    let x_padding = 20;
    let y_padding = 20;
    let total_width = max_x - min_x;
    let total_height = max_y - min_y;
    let wiggler_width = total_width as i32 / wiggler_columns - x_padding;
    let wiggler_height = total_height as i32 / wiggler_rows - y_padding;
    let tail_length = wiggler_height as f32 * 3.5 / 5.0;
    // println!("{:?}", tail_length);
    let draw = _app.draw();
    let x_offset: f32 = 0  as f32;
    let y_offset: f32 = 0 as f32;
    let outer_color = PINK; //hsl (x_offset.clone() / 360.0, 1.0, 20.0);
    let inner_color = GREEN; //hsl (y_offset.clone() / 360.0, 2.0, 20.0);
    let outer_points = get_wiggler_points(&draw, wiggler_width, 1.5, 0.9, tail_length as usize, &_app.time, &x_offset, &y_offset);
    let inner_points = get_wiggler_points(&draw, wiggler_width.clone() / 2, 1.5, 0.9, tail_length as usize, &_app.time, &x_offset, &y_offset);
    let outer_edging = geometry_functions::semi_circle_edge(&outer_points, 24);
    // draw.polygon()
    //     .points(outer_edging.clone())
    //     .color(YELLOW);
    // draw.polygon()
    //     .points(outer_points.clone())
    //     .color(outer_color);
    // draw.polygon()
    //     .points(inner_points.clone())
    //     .color(inner_color);

    for i in 0..(&wiggler_rows * &wiggler_columns) {
        let row_number = (i / &wiggler_columns) as i32;
        let column_number = i % &wiggler_columns;
        let x_offset: f32 = min_x.clone() + (wiggler_width.clone() + x_padding.clone()) as f32 / 2.0 + (column_number * wiggler_width.clone() + x_padding.clone() * column_number)  as f32;
        let y_offset: f32 = min_y.clone() + (wiggler_height.clone() + y_padding.clone()) as f32 / 2.0 + (row_number * wiggler_height.clone()  + y_padding.clone() * row_number.clone()) as f32;
        let outer_edging_size = outer_edging.len();
        let mut translated_outer_edging = Vec::with_capacity(outer_edging_size.clone());
        for i in 0..outer_edging_size {
            translated_outer_edging.push(pt2(outer_edging[i].x + &x_offset, outer_edging[i].y + &y_offset));
        }
        draw.polygon()
            .points(translated_outer_edging)
            .color(YELLOW);
        let outer_points_size = outer_points.len();
        let mut translated_outer_points = Vec::with_capacity(outer_points_size);
        for i in 0..outer_points_size {
            translated_outer_points.push(pt2(outer_points[i].x + &x_offset, outer_points[i].y + &y_offset));
        }
        draw.polygon()
            .points(translated_outer_points)
            .color(outer_color);
        let inner_points_size = inner_points.len();
        let mut translated_inner_points = Vec::with_capacity(inner_points_size);
        for i in 0..inner_points_size {
            translated_inner_points.push(pt2(inner_points[i].x + &x_offset, inner_points[i].y + &y_offset));
        }
        draw.polygon()
            .points(translated_inner_points)
            .color(inner_color);
    }

    draw.to_frame(_app, &frame).unwrap();
}

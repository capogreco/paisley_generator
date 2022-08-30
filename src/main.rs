extern crate nannou;
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

fn view(_app: &App, _model: &Model, frame: Frame){
    frame.clear(PINK);

    let draw = _app.draw();

    draw.ellipse().width(100.0).height(100.0);

    let time = _app.time;

    // Width?
    let wid = 100;
    // Rate of Wiggle
    let rat: f64 = 1.5;
    // Turn
    let trn: f64 =  0.5;
    let twirlAngle = time as f64 * TAU as f64* rat as f64;
    let twirl = twirlAngle.sin() as f32 * PI / 2.0;

    const tailLength: usize = 300;
    let mut points1: [Vec2; tailLength] = [pt2(0.0,0.0); tailLength];
    let mut points2: [Vec2; tailLength] = [pt2(0.0,0.0); tailLength];
    let mut points3: [Vec2; tailLength] = [pt2(0.0,0.0); tailLength];
    for i in 1..tailLength {
        let p = i as f64 / tailLength as f64;
        let w = wid as f64 * (p * PI as f64 / 2.0 as f64).cos().pow(4);
        let theta = p * trn as f64 * twirl as f64;
        let angle = (PI / 2.0) as f64 + theta;
        if i < 20 {
            println!("i: {}, w: {}, theta: {}, angle: {}", i, w, theta, angle)
        }
        let x1 = angle.cos() as f32 * i as f32;
        let y1 = angle.sin() as f32 * i as f32;
        // let p1: Vec2 = Vec2::from([x, y]);
        let p1 = pt2(x1, y1);
        points1[i] = p1;
        let x2 = (angle as f32 + PI/2.0).cos() as f32 * w as f32 / 2.0 as f32;
        let y2 = (angle as f32 + PI/2.0).sin() as f32 * w as f32 / 2.0 as f32;
        // let p1: Vec2 = Vec2::from([x, y]);
        let p2 = pt2(x2, y2) + p1;
        points2[i] = p2;

        let x3 = (angle as f32 - PI/2.0).cos() as f32 * w as f32 / 2.0 as f32;
        let y3 = (angle as f32 - PI/2.0).sin() as f32 * w as f32 / 2.0 as f32;
        // let p1: Vec2 = Vec2::from([x, y]);
        let p3 = pt2(x3, y3) + p1;
        points3[i] = p3;
        // points.push((x as f32, y as f32));
        // print!("{:?}", p1);
        // print!("hi");
        // p1;
    }
    // Create a polyline builder. Hot-tip: polyline is short-hand for a path that is
    // drawn via "stroke" tessellation rather than "fill" tessellation.
    // draw.polyline()
    //     .weight(3.0)
    //     .points(points1);
    draw.polyline()
        .weight(3.0)
        .points(points2);
    draw.polyline()
        .weight(3.0)
        .points(points3);
    // if time < 5.0 {
    //     println!("points {:?}", points);
    // }

    // println!("bye");

    draw.to_frame(_app, &frame).unwrap();
}

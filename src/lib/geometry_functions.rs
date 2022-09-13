extern crate nannou;
use nannou::prelude::*;
use nannou::prelude::geom::Vec2;

fn semi_circle(start_point :: Vec2, end_point: Vec2) -> [Vec2: 180] {
    let diameter = end_point - start_point;
    let radius = diameter * 0.5;
    let mid_point = start_point + radius;

    let mut points = [Vec2: 180] = [pt2(0.0, 0.0); 180];

    for i in 0..180 {
        let angle = i * PI / 180;
        let r = radius.clone();
        r.rotate (angle);
        let p = mid_point + r;
        points[i] = p;
    }

    return points;
}

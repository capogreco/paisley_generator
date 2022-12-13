extern crate nannou;
use nannou::prelude::*;
use nannou::prelude::geom::Vec2;

pub fn semi_circle_edge(points: &Vec<Vec2>, amount: u16) -> Vec<Vec2> {
    let mut new_points = Vec::with_capacity(amount as usize * 180);
    let interval = points.len() / amount as usize;
    for i in 0..(amount) {
        let end   = points[i as usize * interval];
        let start = points[(i + 1)as usize * interval];
        let new_semi  = semi_circle(start, end);
        for p in new_semi.iter() {
            new_points.push(p.clone());
        }
    }
    new_points
} 

pub fn semi_circle(start_point: Vec2, end_point: Vec2) -> Vec<Vec2> {
    let diameter = end_point - start_point;
    let radius = diameter * 0.5;
    let mid_point = start_point + radius;

    let mut points = Vec::with_capacity(180);

    for i in 0..180 {
        if i % 5 != 0 {
            continue;
        }
        let angle = i as f32 * PI / 180.0;
        // let r = radius.clone();
        let r = radius.rotate(angle);
        // r.rotate (angle);
        let p = mid_point + r;
        // println!("i: {}, p: {}", i, p);
        // points[i] = p;
        points.push(p)
    }

    points
}


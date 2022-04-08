use crate::{meta_ball, vertex};

/*     0
    1 --- 2
  3 |     | 1
    8 --- 4
       2     */

const LINKS: &'static [&[[i32; 2]]; 14] = &[
    &[[0, 3]],
    &[[0, 1]],
    &[[1, 3]],
    &[[1, 2]],
    &[[0, 3], [1, 2]],
    &[[0, 2]],
    &[[2, 3]],
    &[[2, 3]],
    &[[0, 2]],
    &[[0, 1], [2, 3]],
    &[[1, 2]],
    &[[1, 3]],
    &[[0, 1]],
    &[[0, 3]],
];

const OFFSETS: &'static [[f32; 4]; 4] = &[
    [0.0, 0.0, 1.0, 0.0],
    [1.0, 0.0, 0.0, 1.0],
    [1.0, 1.0, -1.0, 0.0],
    [0.0, 1.0, 0.0, -1.0],
];

fn build_vertex(i: i32, x: f32, y: f32, w: f32, h: f32, t: f32, values: &[f32; 4]) -> vertex::Vertex {
    let v0 = values[i as usize];
    let v1 = values[((i + 1) % 4) as usize];
    let m0 = v0;
    let m1 = v1;
    let off = (t - m0) / (m1 - m0);
    let [off_x, off_y, fx, fy] = OFFSETS[i as usize];
    return vertex::Vertex {
        position: [x + off_x * w + off * fx * w, y + off_y * h + off * fy * h]
    };
}

pub fn march_at(x: f32, y: f32, w: f32, h: f32, balls: &[meta_ball::MetaBall]) -> Vec<[vertex::Vertex; 2]> {
    let f = |x: f32, y: f32| {
        let mut r = 0.0;
        for ball in balls {
            r += ball.f(x, y) * ball.radius;
        }
        return r;
    };

    let t = 4.0;
    let mut k: i8 = 0;

    let values = [
        f(x, y),
        f(x + w, y),
        f(x + w, y + h),
        f(x, y + h),
    ];

    for i in 0..4 {
        if values[i] < t {
            k |= 1 << i;
        }
    }

    if k == 0 || k == 15 {
        return vec![];
    }

    let lines_def = LINKS[(k - 1) as usize];

    let mut lines: Vec<[vertex::Vertex; 2]> = vec![];

    for [i0, i1] in lines_def {
        let start = build_vertex(*i0, x, y, w, h, t, &values);
        let end = build_vertex(*i1, x, y, w, h, t, &values);
        lines.push([start, end]);
    }
    return lines;
}

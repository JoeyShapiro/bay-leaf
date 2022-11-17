use crate::point;
use crate::draw_line;

// this will go counter-clockwise
pub struct Triangle {
    pub a: point::Point,
    pub b: point::Point,
    pub c: point::Point
}

impl Clone for Triangle {
    fn clone(&self) -> Triangle {
        *self
    }
}

impl Copy for Triangle { }

pub fn draw_triangle(tri: Triangle) {
    draw_line(tri.a, tri.b);
    draw_line(tri.b, tri.c);
    draw_line(tri.c, tri.a);
}

pub fn triangle_project(tri: Triangle, width: f64, height: f64, perspective_matrix: [[f64; 4]; 4]) -> Triangle {
    let tri_projected = Triangle {
        a: point::project_point(tri.a, width, height, perspective_matrix),
        b: point::project_point(tri.b, width, height, perspective_matrix),
        c: point::project_point(tri.c, width, height, perspective_matrix)
    };

    return tri_projected;
}

pub fn triangle_rot(tri: Triangle, theta_x: f64, theta_y: f64, theta_z: f64) -> Triangle {
    let mut a = point::point_rot_z(tri.a, theta_z);
    a = point::point_rot_x(a, theta_x);
    a = point::point_rot_y(a, theta_y);

    let mut b = point::point_rot_z(tri.b, theta_z);
    b = point::point_rot_x(b, theta_x);
    b = point::point_rot_y(b, theta_y);

    let mut c = point::point_rot_z(tri.c, theta_z);
    c = point::point_rot_x(c, theta_x);
    c = point::point_rot_y(c, theta_y);

    let tri_rot = Triangle {
        a: a,
        b: b,
        c: c
    };

    return tri_rot;
}
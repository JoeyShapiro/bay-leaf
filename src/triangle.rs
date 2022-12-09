use ncurses::mvprintw;

use crate::draw_point;
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

fn find_max_x(tri: Triangle) -> f64 {
    if tri.a.x > tri.b.x && tri.a.x > tri.c.x {
        return tri.a.x;
    } else if tri.b.x > tri.a.x && tri.b.x > tri.c.x {
        return tri.b.x;
    }
    return tri.c.x;
}

fn find_min_x(tri: Triangle) -> f64 {
    if tri.a.x < tri.b.x && tri.a.x < tri.c.x {
        return tri.a.x;
    } else if tri.b.x < tri.a.x && tri.b.x < tri.c.x {
        return tri.b.x;
    }
    return tri.c.x;
}

pub fn find_max_y(tri: Triangle) -> f64 {
    if tri.a.y > tri.b.y && tri.a.y > tri.c.y {
        return tri.a.y;
    } else if tri.b.y > tri.a.y && tri.b.y > tri.c.y {
        return tri.b.y;
    }
    return tri.c.y;
}

pub fn find_min_y(tri: Triangle) -> f64 {
    if tri.a.y < tri.b.y && tri.a.y < tri.c.y {
        return tri.a.y;
    } else if tri.b.y < tri.a.y && tri.b.y < tri.c.y {
        return tri.b.y;
    }
    return tri.c.y;
}

pub fn get_line(p1: point::Point, p2: point::Point) -> Vec<point::Point> {
    let mut points: Vec<point::Point> = Vec::new();

    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    
    let min_x = if p1.x < p2.x { p1.x } else { p2.x }.floor() as i32;
    let max_x = if p1.x < p2.x { p2.x } else { p1.x }.ceil() as i32;
    
    if min_x == max_x { // if inifite slope
        let min_y = if p1.y < p2.y { p1.y } else { p2.y }.floor() as i32;
        let max_y = if p1.y < p2.y { p2.y } else { p1.y }.ceil() as i32;

        for y in min_y..max_y {
            points.push(point::Point { x: min_x as f64, y: y as f64, z: 0.0, w: 0.0 }); // the x we choose doesnt matter; they are all the same
        }
    } else {
        for x in min_x..max_x {
            let y = p1.y + dy * (x as f64 - p1.x) / dx;
            points.push(point::Point { x: x as f64, y, z: 0.0, w: 0.0 });
        }
    }

    return points;
}

pub fn draw_triangle(tri: Triangle) {
    // draw wireframe (redunant for fill)
    // draw_line(tri.c, tri.a);
    // draw_line(tri.a, tri.b);
    // draw_line(tri.b, tri.c);

    // draw vertices
    // draw_point(tri.a);
    // draw_point(tri.b);
    // draw_point(tri.c);

    // draw the triangle and fill it in (super complex)
    // get the ranges
    let max_y = find_max_y(tri) as i32;
    let min_y = find_min_y(tri) as i32;

    let mut points = get_line(tri.a, tri.b);
    points.append(&mut get_line(tri.b, tri.c));
    points.append(&mut get_line(tri.c, tri.a));

    // get the ranges (scanline algorithm)
    // pair = [ y, min_x, max_x ]
    let mut pairs: Vec<[i32; 3]> = Vec::new();
    for y in min_y..max_y {
        let mut pair = [y, 10000, 0];
        // get min
        for p in &points {
            if p.y as i32 == y {
                if (p.x as i32) < pair[1] {
                    pair[1] = p.x as i32;
                }
                if (p.x as i32) > pair[2] {
                    pair[2] = p.x as i32;
                }
            }
        }

        pairs.push(pair);
    }

    // draw the pixels (finally)
    for pair in pairs {
        for x in pair[1]..pair[2] {
            mvprintw(pair[0], x, "x");
        }
    }
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

// converts triangle to player space
pub fn triangle_world_to_camera_space(camera: point::Point, triangle: Triangle) -> Triangle {
    return Triangle {
        a: point::point_to_camera_space(triangle.a, camera),
        b: point::point_to_camera_space(triangle.b, camera),
        c: point::point_to_camera_space(triangle.c, camera), // this was a b and caused a major problem
    }
}

pub fn triangle_local_to_world(tri: Triangle, pos: point::Point) -> Triangle {
    return Triangle {
        a: point::point_to_world(tri.a, pos),
        b: point::point_to_world(tri.b, pos),
        c: point::point_to_world(tri.c, pos),
    }
}

pub fn triangle_orbit_cam(camera: point::Point, triangle: Triangle, theta: f64) -> Triangle {
    return Triangle {
        a: point::point_orbit_cam_x(triangle.a, camera, theta),
        b: point::point_orbit_cam_x(triangle.b, camera, theta),
        c: point::point_orbit_cam_x(triangle.b, camera, theta)
    }
}

pub fn triangle_mat_mul(tri: Triangle, mat: [[f64; 4]; 4]) -> Triangle {
    return Triangle {
        a: point::matrix_mul_3d(tri.a, mat),
        b: point::matrix_mul_3d(tri.b, mat),
        c: point::matrix_mul_3d(tri.c, mat),
    }
}
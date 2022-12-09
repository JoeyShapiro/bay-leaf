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
// ($($args:expr),*) => {{
//     let result = 0;
//     $(
//         let result = result + $args;
//     )*
//     result
// }}
macro_rules! min {
    ($($args:expr),*) => {{
        let result = 100000;
        $(
            let result = if result < $args { result } else { $args };
        )*
        result
    }}
    // ($x:expr) => ( $x );
    // ($x:expr, $($xs:expr),+) => {
    //     let result = 0;
    //     {
    //         let m = min!( $($xs),+ );
    //         let result = if $x < m { $x } else { m };
    //         result;
    //     }
    // };
}

macro_rules! max {
    ($($args:expr),*) => {{
        let result = 0;
        $(
            let result = if result > $args { result } else { $args };
        )*
        result
    }}
    // ($x:expr) => ( $x );
    // ($x:expr, $($xs:expr),+) => {
    //     let result = 0;
    //     {
    //         let m = min!( $($xs),+ );
    //         let result = if $x < m { $x } else { m };
    //         result;
    //     }
    // };
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

// these are custom made and cleaner than Points
// they must also be converted to ints a soon as possible
#[derive(Copy, Clone)]
pub struct Pixel {
    x: i32,
    y: i32
}

fn determine_line(p1: Pixel, p2: Pixel) -> Vec<Pixel> {
    let slope = (p2.y - p1.y) as f64 / (p2.x - p1.x) as f64; // this must be f64

    let points: Vec<Pixel> = if slope > 1.0 {
            get_line(p1.y, p1.x, p2.y, p2.x, true, false)
        } else if 0.0 <= slope && slope <= 1.0 {
            get_line(p1.x, p1.y, p2.x, p2.y, false, false)
        } else if -1.0 <= slope && slope < 0.0 {
            get_line(p1.y, p1.x, p2.y, p2.x, false, true)
        } else { // slope < -1
            get_line(-p1.y, p1.x, -p2.y, p2.x, true, true)
        };

    return points;
}

pub fn get_line(x1: i32, y1: i32, x2: i32, y2: i32, large: bool, neg: bool) -> Vec<Pixel> {
    let mut pixels: Vec<Pixel> = Vec::new();

    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let mut x = std::cmp::min(x1, x2);
    let mut y = std::cmp::min(y1, y2);
    let mut p = 2 * dy - dx;

    let max_x = std::cmp::max(x1, x2);

    // for x in std::cmp::min(p1.x, p2.x)..max_x {
        
    // }

    while x <= max_x {
        let nx = if large { y } else { x };
        let mut ny = if large { x } else { y };
        ny = if neg { -ny } else { ny };
        pixels.push(Pixel {
            x: nx,
            y: ny
        });

        if p >= 0 {
            y += 1;
            p += 2 * dy - 2 * dx;
        } else {
            p += 2 * dy;
        }

        x += 1;
    }

    println!("new");
    for pixel in pixels.iter() {
        println!("{}, {}", pixel.x, pixel.y);
    }

    return pixels;
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
    let a = Pixel { x: tri.a.x.round() as i32, y: tri.a.y.round() as i32 };
    let b = Pixel { x: tri.b.x.round() as i32, y: tri.b.y.round() as i32 };
    let c = Pixel { x: tri.c.x.round() as i32, y: tri.c.y.round() as i32 };

    // seems best to conver to pixel here
    let mut points = determine_line(a, b);
    points.append(&mut determine_line(b, c));
    points.append(&mut determine_line(c, a));

    println!("pixels");
    for p in points.iter() {
        println!("{}, {}", p.x, p.y);
    }

    let min_y = min!(a.y, b.y, c.y);
    let max_y = max!(a.y, b.y, c.y);

    // get the ranges (scanline algorithm)
    // pair = [ y, min_x, max_x ]
    let mut pairs: Vec<[i32; 3]> = Vec::new();
    for y in min_y..max_y+1 {
        let mut pair = [y, 10000, 0];
        // get min
        for p in &points {
            if p.y == y {
                if (p.x) < pair[1] {
                    pair[1] = p.x;
                }
                if (p.x) > pair[2] {
                    pair[2] = p.x;
                }
            }
        }

        pairs.push(pair);
    }

    // draw the pixels (finally)
    for pair in pairs {
        println!("{:?}", pair);
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
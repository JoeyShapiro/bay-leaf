use ncurses::*;
use std::fmt;

const W_SIZE: i32 = 8;

fn main() {
    println!("Hello, world!");

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut z: i32 = 0;
    let mut theta: i32 = 0;
    let mut phi: i32 = 0;
    let fov: i32 = 90;
    let mut count = 0.0;
    
    initscr();
    
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    let projection_matrix = create_projection_matrix(max_y as f64 / max_x as f64, 90.0, 0.1, 1000.0);
    let mut triangles: Vec<Triangle> = Vec::new();

    let a = Point { x: 0.0, y: 1.0, z: 0.0, w: 1.0 };
    let b = Point { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
    let c = Point { x: 1.0, y: 0.0, z: 0.0, w: 1.0 };
    let d = Point { x: 1.0, y: 1.0, z: 0.0, w: 1.0 };
    let e = Point { x: 0.0, y: 0.0, z: 1.0, w: 1.0 };
    let f = Point { x: 0.0, y: 1.0, z: 1.0, w: 1.0 };
    let g = Point { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
    let h = Point { x: 1.0, y: 0.0, z: 1.0, w: 1.0 };

    // let a = Point { x: 1.0, y: 3.0, z: 1.0, w: 1.0 };
    // let b = Point { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
    // let c = Point { x: 3.0, y: 1.0, z: 1.0, w: 1.0 };
    // let d = Point { x: 3.0, y: 3.0, z: 1.0, w: 1.0 };
    // let e = Point { x: 1.0, y: 1.0, z: 3.0, w: 1.0 };
    // let f = Point { x: 1.0, y: 3.0, z: 3.0, w: 1.0 };
    // let g = Point { x: 3.0, y: 3.0, z: 3.0, w: 1.0 };
    // let h = Point { x: 3.0, y: 1.0, z: 3.0, w: 1.0 };

    // south
    triangles.push(Triangle { a: a, b: b, c: c });
    triangles.push(Triangle { a: a, b: d, c: c });
    // east
    triangles.push(Triangle { a: f, b: e, c: b });
    triangles.push(Triangle { a: f, b: a, c: b });
    // north
    triangles.push(Triangle { a: g, b: h, c: f });
    triangles.push(Triangle { a: g, b: e, c: f });
    // west
    triangles.push(Triangle { a: c, b: d, c: h });
    triangles.push(Triangle { a: c, b: g, c: h });
    // top
    triangles.push(Triangle { a: f, b: a, c: c });
    triangles.push(Triangle { a: f, b: g, c: c });
    // bottom
    triangles.push(Triangle { a: h, b: d, c: b });
    triangles.push(Triangle { a: h, b: e, c: b });

    // points.push(Point { x: 5.0, y: 5.0, z: 5.0, w: 1.0 });
    // points.push(Point { x: 5.0, y: 11-3.0, z: 5.0, w: 1.0 });
    // points.push(Point { x: 10.0, y: 10.0, z: 5.0, w: 1.0 });
    // points.push(Point { x: 10.0, y: 5.0, z: 5.0, w: 1.0 });
    // points.push(Point { x: 5.0, y: 5.0, z: 10.0, w: 1.0 });
    // points.push(Point { x: 5.0, y: 10.0, z: 10.0, w: 1.0 });
    // points.push(Point { x: 10.0, y: 10.0, z: 10.0, w: 1.0 });
    // points.push(Point { x: 10.0, y: 5.0, z: 10.0, w: 1.0 });

    // for triangle in triangles.iter() {
    //     let tr = triangle_rot(*triangle, 0.0, 0.0, 0.0);
    //     let tn = triangle_project(tr, max_x as f64, max_y as f64, projection_matrix);
    //     draw_triangle(tn);
    // }

    // draw_line(tn.a, tn.c);

    loop {
        let input = getch();
        
        clear();
        
        if input == 119 && x < W_SIZE-1 { // 
            x+=1;
        } else if input == 115 && x > 0 {
            x-=1;
        } else if input == 97 && y > 0 { // 
            y-=1;
        } else if input == 100 && y < W_SIZE-1 {
            y+=1;
        } else if input == 32 && z < W_SIZE-1 { // 
            z+=1;
        } else if input == 118 && z > 0 {
            z-=1
        } else if input == 67 { // 
            theta+=90;
            theta%=360;
        } else if input == 68 {
            theta-=90;
            theta = theta.rem_euclid(360); // % is actually rem. for neg, this has a different result
        } else if input == 65 {
            phi+=90;
            phi%=360;
        } else if input == 66 {
            phi-=90;
            phi = phi.rem_euclid(360);
        }

        for triangle in triangles.iter() {
            let tr = triangle_rot(*triangle, count*0.5, 0.0, count); // count*0.5, 0.0, count
            let tn = triangle_project(tr, max_x as f64, max_y as f64, projection_matrix);

            draw_triangle(tn);
            // for screen_y in 0..max_y {
            //     for screen_x in 0..max_x {
            //         if screen_x == tn.a.x.floor() as i32 && screen_y == tn.a.y.floor() as i32 {
            //             mvprintw(screen_y, screen_x, "x");
            //         }
            //         if screen_x == tn.b.x.floor() as i32 && screen_y == tn.b.y.floor() as i32 {
            //             mvprintw(screen_y, screen_x, "x");
            //         }
            //         if screen_x == tn.c.x.floor() as i32 && screen_y == tn.c.y.floor() as i32 {
            //             mvprintw(screen_y, screen_x, "x");
            //         }
            //     }
            // }
        }

        mvprintw(0, 0, &("input: ".to_owned()+&input.to_string()));
        mvprintw(1, 0, &("x: ".to_owned()+&x.to_string()));
        mvprintw(2, 0, &("y: ".to_owned()+&y.to_string()));
        mvprintw(3, 0, &("z: ".to_owned()+&z.to_string()));
        mvprintw(4, 0, &("theta: ".to_owned()+&theta.to_string()));
        mvprintw(5, 0, &("phi: ".to_owned()+&phi.to_string()));
        mvprintw(6, 0, &("fov: ".to_owned()+&fov.to_string()));
        mvprintw(6, 0, &("count: ".to_owned()+&count.to_string()));

        count += 10.0;
        refresh();
    }
}

fn create_projection_matrix(a: f64, fov: f64, znear: f64, zfar: f64) -> [[f64; 4]; 4] {
    let mut proj_mat = [[0.0f64; 4]; 4];

    let f = 1.0 / (fov / 2.0).to_radians().tan(); // without radians it spirals the cam
    let scale = zfar / (zfar - znear);

    proj_mat[0][0] = a*f;
    proj_mat[1][1] = f;
    proj_mat[2][2] = scale;
    proj_mat[2][3] = 1.0;
    proj_mat[3][2] = -scale*znear;

    return proj_mat
}

struct Point {
    x: f64,
    y: f64,
    z: f64,
    w: f64
}

impl fmt::Display for Point {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "x: {}, y: {}, z: {}, w: {}", self.x, self.y, self.z, self.w)
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        *self
    }
}

impl Copy for Point { }

fn matrix_mul(p: Point, mat: [[f64; 4]; 4]) -> Point {
    let point_prime = Point { // this needs +3 to z
        x: p.x * mat[0][0] + p.y * mat[1][0] + (p.z+3.0) * mat[2][0] + p.w * mat[3][0], 
        y: p.x * mat[0][1] + p.y * mat[1][1] + (p.z+3.0) * mat[2][1] + p.w * mat[3][1], 
        z: p.x * mat[0][2] + p.y * mat[1][2] + (p.z+3.0) * mat[2][2] + p.w * mat[3][2], 
        w: p.x * mat[0][3] + p.y * mat[1][3] + (p.z+3.0) * mat[2][3] + p.w * mat[3][3]
    };
    
    return point_prime;
}

fn point_normalize_perspective(p: Point, width: f64, height: f64) -> Point {
    if (p.w * 100.0).round() / 100.0 == 0.0 {
        return p;
    }

    let point_normalized = Point {
        x: (p.x / p.w + 1.0) * 0.5 * width,
        y: (p.y / p.w + 1.0) * 0.5 * height,
        z: p.z / p.w,
        w: 1.0
    };

    return point_normalized;
}

fn project_point(p: Point, width: f64, height: f64, perspective_matrix: [[f64; 4]; 4]) -> Point {
    return point_normalize_perspective(matrix_mul(p, perspective_matrix), width, height);
}

fn point_rot_z(p: Point, theta: f64) -> Point {
    let c = theta.to_radians().cos();
    let s = theta.to_radians().sin();

    let point_rot_z = Point {
        x: p.x * c - p.y * s + p.z * 0.0 + p.w * 0.0,
        y: p.x * s + p.y * c + p.z * 0.0 + p.w * 0.0,
        z: p.x * 0.0 + p.y * 0.0 + p.z * 1.0 + p.w * 0.0,
        w: p.x * 0.0 + p.y * 0.0 + p.z * 0.0 + p.w * 1.0
    };

    return point_rot_z;
}

fn point_rot_x(p: Point, theta: f64) -> Point {
    let c = theta.to_radians().cos();
    let s = theta.to_radians().sin();

    let point_rot_x = Point {
        x: p.x * 1.0 + p.y * 0.0 + p.z * 0.0 + p.w * 0.0,
        y: p.x * 0.0 + p.y * c - p.z * s + p.w * 0.0,
        z: p.x * 0.0 + p.y * s + p.z * c + p.w * 0.0,
        w: p.x * 0.0 + p.y * 0.0 + p.z * 0.0 + p.w * 1.0,
    };

    return point_rot_x;
}

fn point_rot_y(p: Point, theta: f64) -> Point {
    let c = theta.to_radians().cos();
    let s = theta.to_radians().sin();

    let point_rot_y = Point {
        x: p.x * c + p.y * 0.0 + p.z * s + p.w * 0.0,
        y: p.x * 0.0 + p.y * 1.0 + p.z * 0.0 + p.w * 0.0,
        z: p.x * -s + p.y * 0.0 + p.z * c + p.w * 0.0,
        w: p.x * 0.0 + p.y * 0.0 + p.z * 0.0 + p.w * 1.0,
    };

    return point_rot_y;
}

// this will go counter-clockwise
struct Triangle {
    a: Point,
    b: Point,
    c: Point
}

impl Clone for Triangle {
    fn clone(&self) -> Triangle {
        *self
    }
}

impl Copy for Triangle { }

// this seems inefficient
fn draw_line(p1: Point, p2: Point) {
    // let m = (p2.y - p1.y) / (p2.x - p1.x);
    // for i in 0..height {
    //     for j in 0..width {
    //         if j as f64 == m*i as f64 {
    //             mvprintw(i, j, "x");
    //         }
    //     }
    // }

    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    
    let min_x = if p1.x < p2.x { p1.x } else { p2.x }.floor() as i32;
    let max_x = if p1.x < p2.x { p2.x } else { p1.x }.ceil() as i32;
    
    // println!("p1: {}; p2: {}", p1, p2);
    // println!("dx: {}, dy: {}", dx, dy);
    if min_x == max_x { // if inifite slope
        let min_y = if p1.y < p2.y { p1.y } else { p2.y }.floor() as i32;
        let max_y = if p1.y < p2.y { p2.y } else { p1.y }.ceil() as i32;

        for y in min_y..max_y {
            mvprintw(y, min_x, "x"); // the x we choose doesnt matter; they are all the same
        }
    } else {
        for x in min_x..max_x {
            let y = p1.y + dy * (x as f64 - p1.x) / dx;
            mvprintw(y as i32, x, "x");
        }
    }
}

fn draw_triangle(tri: Triangle) {
    draw_line(tri.a, tri.b);
    draw_line(tri.b, tri.c);
    draw_line(tri.c, tri.a);
}

fn triangle_project(tri: Triangle, width: f64, height: f64, perspective_matrix: [[f64; 4]; 4]) -> Triangle {
    let tri_projected = Triangle {
        a: project_point(tri.a, width, height, perspective_matrix),
        b: project_point(tri.b, width, height, perspective_matrix),
        c: project_point(tri.c, width, height, perspective_matrix)
    };

    return tri_projected;
}

fn triangle_rot(tri: Triangle, theta_x: f64, theta_y: f64, theta_z: f64) -> Triangle {
    let mut a = point_rot_z(tri.a, theta_z);
    a = point_rot_x(a, theta_x);
    a = point_rot_y(a, theta_y);

    let mut b = point_rot_z(tri.b, theta_z);
    b = point_rot_x(b, theta_x);
    b = point_rot_y(b, theta_y);

    let mut c = point_rot_z(tri.c, theta_z);
    c = point_rot_x(c, theta_x);
    c = point_rot_y(c, theta_y);

    let tri_rot = Triangle {
        a: a,
        b: b,
        c: c
    };

    return tri_rot;
}

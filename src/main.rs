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

    let mut points: Vec<Point> = Vec::new();
    let projection_matrix = create_projection_matrix(max_y as f64 / max_x as f64, 90.0, 0.1, 1000.0);

    points.push(Point { x: 0.0, y: 0.0, z: 0.0, w: 1.0 });
    points.push(Point { x: 0.0, y: 1.0, z: 0.0, w: 1.0 });
    points.push(Point { x: 1.0, y: 1.0, z: 0.0, w: 1.0 });
    points.push(Point { x: 1.0, y: 0.0, z: 0.0, w: 1.0 });
    points.push(Point { x: 0.0, y: 0.0, z: 1.0, w: 1.0 });
    points.push(Point { x: 0.0, y: 1.0, z: 1.0, w: 1.0 });
    points.push(Point { x: 1.0, y: 1.0, z: 1.0, w: 1.0 });
    points.push(Point { x: 1.0, y: 0.0, z: 1.0, w: 1.0 });

    // points.push(Point { x: 5.0, y: 5.0, z: 5.0, w: 1.0 });
    // points.push(Point { x: 5.0, y: 10.0, z: 5.0, w: 1.0 });
    // points.push(Point { x: 10.0, y: 10.0, z: 5.0, w: 1.0 });
    // points.push(Point { x: 10.0, y: 5.0, z: 5.0, w: 1.0 });
    // points.push(Point { x: 5.0, y: 5.0, z: 10.0, w: 1.0 });
    // points.push(Point { x: 5.0, y: 10.0, z: 10.0, w: 1.0 });
    // points.push(Point { x: 10.0, y: 10.0, z: 10.0, w: 1.0 });
    // points.push(Point { x: 10.0, y: 5.0, z: 10.0, w: 1.0 });

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

        count += 10.0;

        for point in points.iter() {
            let mut pr = point_rot_z(*point, count);
            pr = point_rot_x(pr, count * 0.5);
            pr = point_rot_y(pr, 0.0);

            let pn = project_point(pr, max_x as f64, max_y as f64, projection_matrix);

            for screen_y in 0..max_y {
                for screen_x in 0..max_x {
                    if screen_x == pn.x.floor() as i32 && screen_y == pn.y.floor() as i32 {
                        mvprintw(screen_y, screen_x, "x");
                    }
                }
            }
        }

        mvprintw(0, 0, &("input: ".to_owned()+&input.to_string()));
        mvprintw(1, 0, &("x: ".to_owned()+&x.to_string()));
        mvprintw(2, 0, &("y: ".to_owned()+&y.to_string()));
        mvprintw(3, 0, &("z: ".to_owned()+&z.to_string()));
        mvprintw(4, 0, &("theta: ".to_owned()+&theta.to_string()));
        mvprintw(5, 0, &("phi: ".to_owned()+&phi.to_string()));
        mvprintw(6, 0, &("fov: ".to_owned()+&fov.to_string()));
        mvprintw(6, 0, &("count: ".to_owned()+&count.to_string()));

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
    // for i in 0..4 { // this gets lucky with (5,5,1,1)
    //     println!("{} {}", i, mat[i][3]);
    //     point_prime.x += p.x * mat[i][0];
    //     point_prime.y += p.y * mat[i][1];
    //     point_prime.z += p.z * mat[i][2];
    //     point_prime.w += p.w * mat[i][3];
    //     println!("{}", point_prime);
    // }

    return point_prime;
}

fn point_normalize_perspective(p: Point, width: f64, height: f64) -> Point {
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

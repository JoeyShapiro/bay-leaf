use std::fmt;

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
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

pub fn project_point(p: Point, width: f64, height: f64, perspective_matrix: [[f64; 4]; 4]) -> Point {
    return point_normalize_perspective(matrix_mul(p, perspective_matrix), width, height);
}

pub fn point_rot_z(p: Point, theta: f64) -> Point {
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

pub fn point_rot_x(p: Point, theta: f64) -> Point {
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

pub fn point_rot_y(p: Point, theta: f64) -> Point {
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

// converts world space to player space
// putting space_to_player is confusing
pub fn point_to_camera_space(p: Point, camera: Point) -> Point {
    return Point { x: p.x - camera.x, y: p.y - camera.y, z: p.z - camera.z, w: 1.0 }
}
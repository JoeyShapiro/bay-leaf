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

pub fn matrix_mul(p: Point, mat: [[f64; 4]; 4]) -> Point {
    let point_prime = Point { // this needs +3 to z
        x: p.x * mat[0][0] + p.y * mat[1][0] + (p.z) * mat[2][0] + p.w * mat[3][0], 
        y: p.x * mat[0][1] + p.y * mat[1][1] + (p.z) * mat[2][1] + p.w * mat[3][1], 
        z: p.x * mat[0][2] + p.y * mat[1][2] + (p.z) * mat[2][2] + p.w * mat[3][2], 
        w: p.x * mat[0][3] + p.y * mat[1][3] + (p.z) * mat[2][3] + p.w * mat[3][3]
    };

    // println!("mat {}", point_prime);
    
    return point_prime;
}

pub fn matrix_mul_3d(p: Point, mat: [[f64; 4]; 4]) -> Point {
    let point_prime = Point {
        x: p.x * mat[0][0] + p.y * mat[1][0] + (p.z) * mat[2][0] + p.w * mat[3][0], 
        y: p.x * mat[0][1] + p.y * mat[1][1] + (p.z) * mat[2][1] + p.w * mat[3][1], 
        z: p.x * mat[0][2] + p.y * mat[1][2] + (p.z) * mat[2][2] + p.w * mat[3][2], 
        w: 1.0
    };
    
    return point_prime;
}

pub fn mat_mul(mat: [[f64; 4]; 4], p: Point) -> Point {
    let point_prime = Point {
        x: mat[0][0] * p.x + mat[0][1] * p.y + mat[0][2] * p.z + mat[0][3] * p.w,
        y: mat[1][0] * p.x + mat[1][1] * p.y + mat[1][2] * p.z + mat[1][3] * p.w,
        z: mat[2][0] * p.x + mat[2][1] * p.y + mat[2][2] * p.z + mat[2][3] * p.w,
        w: 1.0
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

pub fn point_to_world(p: Point, pos: Point) -> Point {
    return Point { x: p.x + pos.x, y: p.y + pos.y, z: p.z + pos.z, w: 1.0 };
}

pub fn point_orbit_cam_x(p: Point, camera: Point, theta: f64) -> Point {
    // X := originX + cos(angle)*radius;
    // Y := originY + sin(angle)*radius;
    // x' := xcos(theta) - ysin(theta)
    // y' := ycos(theta) - xsin(theta)

    return Point { 
        x: p.x,
        y: p.y * theta.to_radians().cos() + p.z * theta.to_radians().sin(),
        z: p.z * theta.to_radians().cos() - p.y * theta.to_radians().sin(),
        w: p.w 
    };
}

fn point_sub(p1: Point, p2: Point) -> Point {
    return Point { x: p1.x - p2.x, y: p1.y - p2.y, z: p1.z - p2.z, w: 1.0 };
}

pub fn point_add(p1: Point, p2: Point) -> Point {
    return Point { x: p1.x + p2.x, y: p1.y + p2.y, z: p1.z + p2.z, w: 1.0 };
}

fn point_dot(p1: Point, p2: Point) -> f64 {
    return p1.x * p2.x + p1.y * p2.y + p1.z * p2.z + 0.0 //p1.w * p2.w;
}

fn point_mul(p: Point, k: f64) -> Point {
    return Point {
        x: p.x * k,
        y: p.y * k,
        z: p.z * k,
        w: 1.0 //p.w * k
    };
}

fn point_norm(p: Point) -> Point {
    let l = p.x * p.x + p.y * p.y + p.z * p.z;
    return Point {
        x: p.x / l,
        y: p.y / l,
        z: p.z / l,
        w: 1.0
    };
}

fn point_cross(p1: Point, p2: Point) -> Point {
    return Point {
        x: p1.y * p2.z - p1.z * p2.y,
        y: p1.z * p2.x - p1.x * p2.z,
        z: p1.x * p2.y - p1.y * p2.x,
        w: 1.0
    };
}

pub fn mat_rot_y(yaw: f64) -> [[f64; 4]; 4] {
    let mut m = [[0.0f64; 4]; 4];
    let y = yaw.to_radians();

    m[0][0] = y.cos();
    m[0][2] = -y.sin();
    m[1][1] = 1.0;
    m[0][2] = y.sin();
    m[2][2] = y.cos();
    m[3][3] = 1.0;

    return m;
}

pub fn point_at(p: Point, target: Point, up: Point) -> [[f64; 4]; 4] {
    let mut m = [[0.0f64; 4]; 4];
    let forward = point_sub(target, p);

    let a = point_mul(forward, point_dot(up, forward));
    let tmp_up = point_sub(up, a);
    let new_up = point_norm(tmp_up);

    let new_right = point_cross(new_up, forward);

    // println!("stuff {} {} {}", new_right, new_up, forward);

    m[0][0] = new_right.x;
    m[1][0] = new_up.x;
    m[2][0] = forward.x;
    m[3][0] = p.x;

    m[0][1] = new_right.y;
    m[1][1] = new_up.y;
    m[2][1] = forward.y;
    m[3][1] = p.y;

    m[0][2] = new_right.z;
    m[1][2] = new_up.z;
    m[2][2] = forward.z;
    m[3][2] = p.z;

    m[0][3] = 0.0;
    m[1][3] = 0.0;
    m[2][3] = 0.0;
    m[3][3] = 1.0;

    return m;
}

pub fn quick_inverse(mat: [[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut m = [[0.0f64; 4]; 4];

    m[0][0] = mat[0][0];
    m[1][0] = mat[0][1];
    m[2][0] = mat[0][2];

    m[0][1] = mat[1][0];
    m[1][1] = mat[1][1];
    m[2][0] = mat[1][2];
    
    m[0][2] = mat[2][0];
    m[1][2] = mat[2][1];
    m[2][2] = mat[2][2];

    m[3][0] = - (mat[3][0] * m[0][0] + mat[3][1] * m[1][0] + mat[3][2] * m[2][0]);
    m[3][1] = - (mat[3][0] * m[0][1] + mat[3][1] * m[1][1] + mat[3][2] * m[2][1]);
    m[3][2] = - (mat[3][0] * m[0][2] + mat[3][1] * m[1][2] + mat[3][2] * m[2][2]);
    m[3][3] = 1.0;

    return m;
}

pub fn quaternion(theta: f64, rotator: Point) -> [[f64; 4]; 4] {
    let mut m = [[0.0f64; 4]; 4];

    let theta_q = theta.to_radians() / 2.0;
    let length = (rotator.x*rotator.x + rotator.y*rotator.y + rotator.z*rotator.z).sqrt();

    // safety check
    if length == 0.0 {
        return m;
    }

    // find the quaterion stuff
    // q = a + bi + cj + dk
    let a = theta_q.cos();
    let b = rotator.x / length * theta_q.sin();
    let c = rotator.y / length * theta_q.sin();
    let d = rotator.z / length * theta_q.sin();

    // create the rotation matrix
    m[0][0] = 1.0 - 2.0 * (c*c + d*d);
    m[0][1] = 2.0 * (b * c - a * d);
    m[0][2] = 2.0 * (b * d + a * c);
    m[0][3] = 0.0;

    m[1][0] = 2.0 * (b*c + a*d);
    m[1][1] = 1.0 - 2.0 * (b*b + d*d);
    m[1][2] = 2.0 * (c*d - a*b);
    m[1][3] = 0.0;

    m[2][0] = 2.0*(b*d - a*c);
    m[2][1] = 2.0*(c*d + a*b);
    m[2][2] = 1.0 - 2.0*(b*b + c*c);
    m[2][3] = 0.0;

    m[3][0] = 0.0;
    m[3][1] = 0.0;
    m[3][2] = 0.0;
    m[3][3] = 1.0;

    return m;
}

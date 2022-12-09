use std::time::Instant;

use ncurses::*;
use point::Point;

use crate::{player::Player, point::{point_rot_y, point_at, quick_inverse, mat_rot_y, matrix_mul, matrix_mul_3d, mat_mul, quaternion}, triangle::{triangle_mat_mul, get_line}};

pub mod point;
pub mod triangle;
pub mod key_listener;
pub mod player;
pub mod obj;

const W_SIZE: i32 = 8;
const MILLIS_PER_TICK: u128 = 10; // 1000 ~= 1sec; note for future me

/* CAMERA MOVEMENT REFERENCE
  -y    z
   |   /
   |  /
   | /
   |/
   +---------- x
*/

fn main() { 
    println!("Hello, world!");
    
    let fov: i32 = 90;
    let mut tick = 0.0;

    let mut tps = 0.0;
    let mut obj = obj::Obj::new("res/untitled.obj".to_string(), Point { x: 0.0, y: 0.0, z: 3.0, w: 1.0 });
    println!("a {}", obj.mesh[0].a);
    println!("b {}", obj.mesh[0].b);
    println!("c {}", obj.mesh[0].c);

    let projection_matrix = create_projection_matrix(24.0 / 80.0, 90.0, 0.1, 1000.0);
    
    let theta = 60.0;
    let pos = Point { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
    let rot = Point { x: 1.0, y: 0.0, z: 0.0, w: 1.0 };
    let q = quaternion(theta, rot);
    // local stuff
    let tr = triangle::triangle_rot(obj.mesh[0], 180.0, 180.0 * 0.5, 0.0); // count*0.5, 0.0, count
    let tw = triangle::triangle_local_to_world(tr, obj.pos);
    println!("tw.a {}", tw.a);
    println!("tw.b {}", tw.b);
    println!("tw.c {}", tw.c);
    // camera stuff
    let tc = triangle::triangle_world_to_camera_space(pos, tw); // apparently this goes AFTER rotation
    println!("tc.a {}", tc.a);
    println!("tc.b {}", tc.b);
    println!("tc.c {}", tc.c);
    // let to = triangle::triangle_mat_mul(tc, q);
    // let ty = triangle::triangle_mat_mul(to, q_y);
    // screen stuff
    let tn = triangle::triangle_project(tc, 80.0, 24.0, projection_matrix);
    println!("tn.a {}", tn.a);
    println!("tn.b {}", tn.b);
    println!("tn.c {}", tn.c);

    triangle::draw_triangle(tn, false);

    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();

    let clock = Instant::now();
    let mut then = clock.elapsed().as_millis();
    
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    let projection_matrix = create_projection_matrix(max_y as f64 / max_x as f64, 90.0, 0.1, 1000.0);
    let mut triangles: Vec<triangle::Triangle> = Vec::new();
    let mut my_key_listener = key_listener::KeyListener::new();
    let mut player: Player = Player { position: point::Point { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, theta: 0.0, phi: 0.0 };

    // [PH] Blender Cube
    // let p0 = point::Point { x: 1.0, y: 1.0, z: -1.0, w: 1.0 };
    // let p1 = point::Point { x: 1.0, y: -1.0, z: -1.0, w: 1.0 };
    // let p2 = point::Point { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
    // let p3 = point::Point { x: 1.0, y: -1.0, z: 1.0, w: 1.0 };
    // let p4 = point::Point { x: -1.0, y: 1.0, z: -1.0, w: 1.0 };
    // let p5 = point::Point { x: -1.0, y: -1.0, z: -1.0, w: 1.0 };
    // let p6 = point::Point { x: -1.0, y: 1.0, z: 1.0, w: 1.0 };
    // let p7 = point::Point { x: -1.0, y: -1.0, z: 1.0, w: 1.0 };

    // triangles.push(triangle::Triangle { a: p0, b: p4, c: p6 });
    // triangles.push(triangle::Triangle { a: p2, b: p0, c: p6 });
    // triangles.push(triangle::Triangle { a: p3, b: p2, c: p6 });
    // triangles.push(triangle::Triangle { a: p7, b: p3, c: p6 });
    // triangles.push(triangle::Triangle { a: p7, b: p6, c: p4 });
    // triangles.push(triangle::Triangle { a: p5, b: p7, c: p4 });
    // triangles.push(triangle::Triangle { a: p5, b: p1, c: p3 });
    // triangles.push(triangle::Triangle { a: p7, b: p5, c: p3 });
    // triangles.push(triangle::Triangle { a: p1, b: p0, c: p2 });
    // triangles.push(triangle::Triangle { a: p3, b: p1, c: p2 });
    // triangles.push(triangle::Triangle { a: p5, b: p4, c: p0 });
    // triangles.push(triangle::Triangle { a: p1, b: p5, c: p0 });

    triangles.append(&mut obj.mesh); // i think bottoms are missing

    loop {
        let now = clock.elapsed().as_millis();

        if !is_next_tick(now, then) {
            continue;
        }

        tps = 1.0 / (now - then) as f64 * 100.0;
        then = now;

        clear();

        player = key_listener::listen(&mut my_key_listener, player);

        // basic movement
        my_key_listener.when_pressing_forward = |mut player| { player.position.z += 1.0; player };
        my_key_listener.when_pressing_backward = |mut player| { player.position.z -= 1.0; player };
        my_key_listener.when_pressing_left = |mut player| { player.position.x -= 1.0; player };
        my_key_listener.when_pressing_right = |mut player| { player.position.x += 1.0; player };
        my_key_listener.when_pressing_up = |mut player| { player.position.y -= 1.0; player };
        my_key_listener.when_pressing_down = |mut player| { player.position.y += 1.0; player };

        // basic turning
        my_key_listener.when_pressing_turn_up = |mut player| { player.phi += 1.0; player.phi %= 360.0; player };
        my_key_listener.when_pressing_turn_down = |mut player| { player.phi -= 1.0; player.phi = player.phi.rem_euclid(360.0); player }; // % is actually rem. for neg, this has a different result
        my_key_listener.when_pressing_turn_right = |mut player| { player.theta += 1.0; player.theta %= 360.0; player };
        my_key_listener.when_pressing_turn_left = |mut player| { player.theta -= 1.0; player.theta = player.theta.rem_euclid(360.0); player };

        // if input == 119 && x < W_SIZE-1 { // 
        //     x+=1;
        // } else if input == 115 && x > 0 {
        //     x-=1;
        // } else if input == 97 && y > 0 { // 
        //     y-=1;
        // } else if input == 100 && y < W_SIZE-1 {
        //     y+=1;
        // } else if input == 32 && z < W_SIZE-1 { // 
        //     z+=1;
        // } else if input == 118 && z > 0 {
        //     z-=1
        // } else if input == 67 { // 
        //     theta+=90;
        //     theta%=360;
        // } else if input == 68 {
        //     theta-=90;
        //     theta = theta.rem_euclid(360); 
        // } else if input == 65 {
        //     phi+=90;
        //     phi%=360;
        // } else if input == 66 {
        //     phi-=90;
        //     phi = phi.rem_euclid(360);
        // }

        let rot = Point { x: 1.0, y: 0.0, z: 0.0, w: 1.0 };
        let q = quaternion(player.phi, rot);
        let rot_y = Point { x: 0.0, y: 1.0, z: 0.0, w: 1.0 };
        let q_y = quaternion(player.theta, rot_y);

        init_pair(1, 14, COLOR_MAGENTA);
        attron(COLOR_PAIR(1));
        for triangle in triangles.iter() {
            // local stuff
            let tr = triangle::triangle_rot(*triangle, tick, tick * 0.5, 0.0); // count*0.5, 0.0, count
            let tw = triangle::triangle_local_to_world(tr, obj.pos);
            // camera stuff
            let tc = triangle::triangle_world_to_camera_space(player.position, tw); // apparently this goes AFTER rotation
            let to = triangle::triangle_mat_mul(tc, q);
            let ty = triangle::triangle_mat_mul(to, q_y);
            // screen stuff
            let tn = triangle::triangle_project(ty, max_x as f64, max_y as f64, projection_matrix);

            if tn.a.x <= tn.b.x && tn.c.x >= tn.b.x {
            }
            triangle::draw_triangle(tn, true);
            // break;
        }

        tick += 1.0;

        init_pair(2, 8, 0);
        attron(COLOR_PAIR(2));
        mvprintw(1, 0, &("x: ".to_owned()+&player.position.x.to_string()));
        mvprintw(2, 0, &("y: ".to_owned()+&player.position.y.to_string()));
        mvprintw(3, 0, &("z: ".to_owned()+&player.position.z.to_string()));
        mvprintw(4, 0, &("theta: ".to_owned()+&player.theta.to_string()));
        mvprintw(5, 0, &("phi: ".to_owned()+&player.phi.to_string()));
        mvprintw(6, 0, &("fov: ".to_owned()+&fov.to_string()));
        mvprintw(7, 0, &("tick: ".to_owned()+&tick.to_string()));
        mvprintw(8, 0, &("tps: ".to_owned()+&tps.to_string()));

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

// this seems inefficient
fn draw_line(p1: point::Point, p2: point::Point) {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    
    let min_x = if p1.x < p2.x { p1.x } else { p2.x }.floor() as i32;
    let max_x = if p1.x < p2.x { p2.x } else { p1.x }.ceil() as i32;
    
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

fn draw_point(p: point::Point) {
    let pp = Point {
        x: p.x.round(),
        y: p.y.round(),
        z: p.z.round(),
        w: p.w.round()
    };

    mvprintw(p.y as i32, p.x as i32, "x");
    mvprintw(p.y as i32 - 1, p.x as i32 + 1, &pp.to_string());
}

fn is_next_tick(now: u128, then: u128) -> bool {
    return now - then >= MILLIS_PER_TICK;
}
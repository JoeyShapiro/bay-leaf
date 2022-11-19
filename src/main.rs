use std::time::Instant;

use ncurses::*;

use crate::player::Player;

pub mod point;
pub mod triangle;
pub mod key_listener;
pub mod player;

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
    
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let clock = Instant::now();
    let mut then = clock.elapsed().as_millis();
    
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    let projection_matrix = create_projection_matrix(max_y as f64 / max_x as f64, 90.0, 0.1, 1000.0);
    let mut triangles: Vec<triangle::Triangle> = Vec::new();
    let mut my_key_listener = key_listener::KeyListener::new();
    let mut player: Player = Player { position: point::Point { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, theta: 0.0, phi: 0.0 };

    let a = point::Point { x: 0.0, y: 1.0, z: 0.0, w: 1.0 };
    let b = point::Point { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
    let c = point::Point { x: 1.0, y: 0.0, z: 0.0, w: 1.0 };
    let d = point::Point { x: 1.0, y: 1.0, z: 0.0, w: 1.0 };
    let e = point::Point { x: 0.0, y: 0.0, z: 1.0, w: 1.0 };
    let f = point::Point { x: 0.0, y: 1.0, z: 1.0, w: 1.0 };
    let g = point::Point { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
    let h = point::Point { x: 1.0, y: 0.0, z: 1.0, w: 1.0 };

    // let a = Point { x: 1.0, y: 3.0, z: 1.0, w: 1.0 };
    // let b = Point { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
    // let c = Point { x: 3.0, y: 1.0, z: 1.0, w: 1.0 };
    // let d = Point { x: 3.0, y: 3.0, z: 1.0, w: 1.0 };
    // let e = Point { x: 1.0, y: 1.0, z: 3.0, w: 1.0 };
    // let f = Point { x: 1.0, y: 3.0, z: 3.0, w: 1.0 };
    // let g = Point { x: 3.0, y: 3.0, z: 3.0, w: 1.0 };
    // let h = Point { x: 3.0, y: 1.0, z: 3.0, w: 1.0 };

    // south
    triangles.push(triangle::Triangle { a: a, b: b, c: c });
    triangles.push(triangle::Triangle { a: a, b: d, c: c });
    // east
    triangles.push(triangle::Triangle { a: f, b: e, c: b });
    triangles.push(triangle::Triangle { a: f, b: a, c: b });
    // north
    triangles.push(triangle::Triangle { a: g, b: h, c: f });
    triangles.push(triangle::Triangle { a: g, b: e, c: f });
    // west
    triangles.push(triangle::Triangle { a: c, b: d, c: h });
    triangles.push(triangle::Triangle { a: c, b: g, c: h });
    // top
    triangles.push(triangle::Triangle { a: f, b: a, c: c });
    triangles.push(triangle::Triangle { a: f, b: g, c: c });
    // bottom
    triangles.push(triangle::Triangle { a: h, b: d, c: b });
    triangles.push(triangle::Triangle { a: h, b: e, c: b });

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
        let now = clock.elapsed().as_millis();

        if !is_next_tick(now, then) {
            continue;
        }

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

        for triangle in triangles.iter() {
            let tc = triangle::triangle_world_to_camera_space(player.position, *triangle);
            let tr = triangle::triangle_rot(tc, player.phi, player.theta, 0.0); // count*0.5, 0.0, count
            let tn = triangle::triangle_project(tr, max_x as f64, max_y as f64, projection_matrix);

            triangle::draw_triangle(tn);
        }

        tick += 1.0;

        mvprintw(1, 0, &("x: ".to_owned()+&player.position.x.to_string()));
        mvprintw(2, 0, &("y: ".to_owned()+&player.position.y.to_string()));
        mvprintw(3, 0, &("z: ".to_owned()+&player.position.z.to_string()));
        mvprintw(4, 0, &("theta: ".to_owned()+&player.theta.to_string()));
        mvprintw(5, 0, &("phi: ".to_owned()+&player.phi.to_string()));
        mvprintw(6, 0, &("fov: ".to_owned()+&fov.to_string()));
        mvprintw(7, 0, &("tick: ".to_owned()+&tick.to_string()));

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

fn is_next_tick(now: u128, then: u128) -> bool {
    return now - then >= MILLIS_PER_TICK;
}
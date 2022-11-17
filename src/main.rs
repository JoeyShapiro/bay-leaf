use ncurses::*;
use device_query::{DeviceQuery, DeviceState, Keycode}; // using this for now, for simplicity

pub mod point;
pub mod triangle;
pub mod key_listener;

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
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    let projection_matrix = create_projection_matrix(max_y as f64 / max_x as f64, 90.0, 0.1, 1000.0);
    let mut triangles: Vec<triangle::Triangle> = Vec::new();
    let mut key_listener = key_listener::KeyListener{..Default::default()};

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
        clear();
        let device_state = DeviceState::new();
        let keys: Vec<Keycode> = device_state.get_keys(); // how does this work ???
        if !keys.is_empty() {
            for (i, key) in keys.iter().enumerate() {
                match key.to_string() {
                    key if key == key_listener.up => key_listener.is_holding_up = true,
                    key if key == key_listener.forward => key_listener.is_holding_forward = true,
                    key if key == key_listener.backward => key_listener.is_holding_backward = true,
                    key if key == key_listener.left => key_listener.is_holding_left = true,
                    key if key == key_listener.right => key_listener.is_holding_right = true,
                    key if key == key_listener.down => key_listener.is_holding_down = true,
                    key if key == key_listener.turn_left => key_listener.is_holding_turn_left = true,
                    key if key == key_listener.turn_right => key_listener.is_holding_turn_right = true,
                    key if key == key_listener.turn_up => key_listener.is_holding_turn_up = true,
                    key if key == key_listener.turn_down => key_listener.is_holding_turn_down = true,

                    _ => _ = mvprintw(i as i32+1, 0, &("key: ".to_owned()+&key.to_string()))
                }
            }
        }

        // forward
        if key_listener.is_holding_forward {
            mvprintw(0, 0, &("Holding forward".to_owned()));
            key_listener.when_pressing_forward;
            key_listener.was_holding_forward = true;
            key_listener.is_holding_forward = false;
        } else if key_listener.was_holding_forward && !key_listener.is_holding_forward { // when the user releases forward
            key_listener.when_released_forward;
            key_listener.was_holding_forward = false;
            mvprintw(0, 0, &("Released forward".to_owned()));
        }

        // backward
        if key_listener.is_holding_backward {
            mvprintw(0, 0, &("Holding backward".to_owned()));
            key_listener.when_pressing_backward;
            key_listener.was_holding_backward = true;
            key_listener.is_holding_backward = false;
        } else if key_listener.was_holding_backward && !key_listener.is_holding_backward { // when the user releases backward
            key_listener.when_released_backward;
            key_listener.was_holding_backward = false;
            mvprintw(0, 0, &("Released backward".to_owned()));
        }

        // left
        if key_listener.is_holding_left {
            mvprintw(0, 0, &("Holding left".to_owned()));
            key_listener.when_pressing_left;
            key_listener.was_holding_left = true;
            key_listener.is_holding_left = false;
        } else if key_listener.was_holding_left && !key_listener.is_holding_left { // when the user releases left
            key_listener.when_released_left;
            key_listener.was_holding_left = false;
            mvprintw(0, 0, &("Released left".to_owned()));
        }

        // right
        if key_listener.is_holding_right {
            mvprintw(0, 0, &("Holding right".to_owned()));
            key_listener.when_pressing_right;
            key_listener.was_holding_right = true;
            key_listener.is_holding_right = false;
        } else if key_listener.was_holding_right && !key_listener.is_holding_right { // when the user releases right
            key_listener.when_released_right;
            key_listener.was_holding_right = false;
            mvprintw(0, 0, &("Released right".to_owned()));
        }

        // up
        if key_listener.is_holding_up {
            mvprintw(0, 0, &("Holding up".to_owned()));
            key_listener.when_pressing_up;
            key_listener.was_holding_up = true;
            key_listener.is_holding_up = false;
        } else if key_listener.was_holding_up && !key_listener.is_holding_up { // when the user releases up
            key_listener.when_released_up;
            key_listener.was_holding_up = false;
            mvprintw(0, 0, &("Released up".to_owned()));
        }

        // down
        if key_listener.is_holding_down {
            mvprintw(0, 0, &("Holding down".to_owned()));
            key_listener.when_pressing_down;
            key_listener.was_holding_down = true;
            key_listener.is_holding_down = false;
        } else if key_listener.was_holding_down && !key_listener.is_holding_down { // when the user releases down
            key_listener.when_released_down;
            key_listener.was_holding_down = false;
            mvprintw(0, 0, &("Released down".to_owned()));
        }

        // turn_left
        if key_listener.is_holding_turn_left {
            mvprintw(0, 0, &("Holding turn_left".to_owned()));
            key_listener.when_pressing_turn_left;
            key_listener.was_holding_turn_left = true;
            key_listener.is_holding_turn_left = false;
        } else if key_listener.was_holding_turn_left && !key_listener.is_holding_turn_left { // when the user releases turn_left
            key_listener.when_released_turn_left;
            key_listener.was_holding_turn_left = false;
            mvprintw(0, 0, &("Released turn_left".to_owned()));
        }

        // turn_right
        if key_listener.is_holding_turn_right {
            mvprintw(0, 0, &("Holding turn_right".to_owned()));
            key_listener.when_pressing_turn_right;
            key_listener.was_holding_turn_right = true;
            key_listener.is_holding_turn_right = false;
        } else if key_listener.was_holding_turn_right && !key_listener.is_holding_turn_right { // when the user releases turn_right
            key_listener.when_released_turn_right;
            key_listener.was_holding_turn_right = false;
            mvprintw(0, 0, &("Released turn_right".to_owned()));
        }

        // turn_up
        if key_listener.is_holding_turn_up {
            mvprintw(0, 0, &("Holding turn_up".to_owned()));
            key_listener.when_pressing_turn_up;
            key_listener.was_holding_turn_up = true;
            key_listener.is_holding_turn_up = false;
        } else if key_listener.was_holding_turn_up && !key_listener.is_holding_turn_up { // when the user releases turn_up
            key_listener.when_released_turn_up;
            key_listener.was_holding_turn_up = false;
            mvprintw(0, 0, &("Released turn_up".to_owned()));
        }

        // turn_down
        if key_listener.is_holding_turn_down {
            mvprintw(0, 0, &("Holding turn_down".to_owned()));
            key_listener.when_pressing_turn_down;
            key_listener.was_holding_turn_down = true;
            key_listener.is_holding_turn_down = false;
        } else if key_listener.was_holding_turn_down && !key_listener.is_holding_turn_down { // when the user releases turn_down
            key_listener.when_released_turn_down;
            key_listener.was_holding_turn_down = false;
            mvprintw(0, 0, &("Released turn_down".to_owned()));
        }
        
        if key_listener.should_stop {
            break;
        }

        refresh();
    }

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
            let tr = triangle::triangle_rot(*triangle, count*0.5, 0.0, count); // count*0.5, 0.0, count
            let tn = triangle::triangle_project(tr, max_x as f64, max_y as f64, projection_matrix);

            triangle::draw_triangle(tn);
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
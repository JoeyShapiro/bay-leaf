use ncurses::*;

fn main() {
    println!("Hello, world!");

    const W_SIZE: i32 = 8;

    let mut world = [[[0u8; W_SIZE as usize]; W_SIZE as usize]; W_SIZE as usize];
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut z: i32 = 0;
    let mut theta: i32 = 0;
    let mut phi: i32 = 0;
    
    world[5][5][5] = 2;
    
    initscr();
    
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    let mut perspective: Vec<Vec<i32>> = vec![vec![Default::default(); max_x as usize]; max_y as usize];
    perspective[5][5] = 1;
    
    loop {
        let input = getch();
        
        clear();
        world[x as usize][y as usize][z as usize] = 0;
        
        if input == 119 && x < W_SIZE-1 { // 
            x+=1;
        } else if input == 115 && x > 0 {
            x-=1;
        } else if input == 97 && y > 0{ // 
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
            theta%=360;
        } else if input == 65 {
            phi+=90;
            phi%=360;
        } else if input == 66 {
            phi-=90;
            phi%=360;
        }

        world[x as usize][y as usize][z as usize] = 1;

        for screen_y in 0..max_y {
            for screen_x in 0..max_x {
                if perspective[screen_y as usize][screen_x as usize] == 1 {
                    mvprintw(screen_y, screen_x, &perspective[screen_y as usize][screen_x as usize].to_string());
                }
            }
        }

        mvprintw(0, 0, &("input: ".to_owned()+&input.to_string()));
        mvprintw(1, 0, &("x: ".to_owned()+&x.to_string()));
        mvprintw(2, 0, &("y: ".to_owned()+&y.to_string()));
        mvprintw(3, 0, &("z: ".to_owned()+&z.to_string()));
        mvprintw(4, 0, &("theta: ".to_owned()+&theta.to_string()));
        mvprintw(5, 0, &("phi: ".to_owned()+&phi.to_string()));

        let distance = f64::sqrt((i32::pow(5 - x, 2) + i32::pow(5 - y, 2) + i32::pow(5 - z, 2)) as f64);

        mvprintw(6, 0, &("d: ".to_owned()+&distance.to_string()));


        refresh();
    }
}

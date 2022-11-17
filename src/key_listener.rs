/*
    class to listen to the keyboard input
    uses ncurses and its own thread
    seems the best way
*/

use ncurses::getch;

pub struct KeyListener {
    pub should_stop: bool,

    // key constants
    pub forward: String,
    pub backward: String,
    pub left: String,
    pub right: String,
    pub up: String,
    pub down: String,
    pub turn_left: String,
    pub turn_right: String,
    pub turn_up: String,
    pub turn_down: String,

    // key pressures
    pub is_holding_space: bool,
    pub was_holding_space: bool,
    pub is_holding_forward: bool,
    pub was_holding_forward: bool,
    pub is_holding_backward: bool,
    pub was_holding_backward: bool,
    pub is_holding_left: bool,
    pub was_holding_left: bool,
    pub is_holding_right: bool,
    pub was_holding_right: bool,
    pub is_holding_up: bool,
    pub was_holding_up: bool,
    pub is_holding_down: bool,
    pub was_holding_down: bool,
    pub is_holding_turn_left: bool,    
    pub was_holding_turn_left: bool,
    pub is_holding_turn_right: bool,
    pub was_holding_turn_right: bool,
    pub is_holding_turn_up: bool,
    pub was_holding_turn_up: bool,
    pub is_holding_turn_down: bool,
    pub was_holding_turn_down: bool,

    // functions
    pub when_pressing_space: fn() -> bool,
    pub when_released_space: fn() -> bool,
    pub when_pressing_forward: fn() -> bool,
    pub when_released_forward: fn() -> bool,
    pub when_pressing_backward: fn() -> bool,
    pub when_released_backward: fn() -> bool,
    pub when_pressing_left: fn() -> bool,
    pub when_released_left: fn() -> bool,
    pub when_pressing_right: fn() -> bool,
    pub when_released_right: fn() -> bool,
    pub when_pressing_up: fn() -> bool,
    pub when_released_up: fn() -> bool,
    pub when_pressing_down: fn() -> bool,
    pub when_released_down: fn() -> bool,
    pub when_pressing_turn_left: fn() -> bool,
    pub when_released_turn_left: fn() -> bool,
    pub when_pressing_turn_right: fn() -> bool,
    pub when_released_turn_right: fn() -> bool,
    pub when_pressing_turn_up: fn() -> bool,
    pub when_released_turn_up: fn() -> bool,
    pub when_pressing_turn_down: fn() -> bool,
    pub when_released_turn_down: fn() -> bool
}

impl KeyListener {

    pub fn dismiss(&mut self) {
        self.should_stop = true;
    }
}

impl Default for KeyListener {
    fn default() -> KeyListener {
        return KeyListener {
            forward: "W".to_owned(),
            backward: "S".to_owned(),
            left: "A".to_owned(),
            right: "D".to_owned(),
            up: "Space".to_owned(),
            down: "V".to_owned(),
            turn_left: "Left".to_owned(),
            turn_right: "Right".to_owned(),
            turn_up: "Up".to_owned(),
            turn_down: "Down".to_owned(),

            is_holding_space: false,
            was_holding_space: false,
            is_holding_forward: false,
            was_holding_forward: false,
            is_holding_backward: false,
            was_holding_backward: false,
            is_holding_left: false,
            was_holding_left: false,
            is_holding_right: false,
            was_holding_right: false,
            is_holding_up: false,
            was_holding_up: false,
            is_holding_down: false,
            was_holding_down: false,
            is_holding_turn_left: false,
            was_holding_turn_left: false,
            is_holding_turn_right: false,
            was_holding_turn_right: false,
            is_holding_turn_up: false,
            was_holding_turn_up: false,
            is_holding_turn_down: false,
            was_holding_turn_down: false,

            should_stop: false,
            when_pressing_space: || return false,
            when_released_space: || return false,
            when_pressing_forward: || return false,
            when_released_forward: || return false,
            when_pressing_backward: || return false,
            when_released_backward: || return false,
            when_pressing_left: || return false,
            when_released_left: || return false,
            when_pressing_right: || return false,
            when_released_right: || return false,
            when_pressing_up: || return false,
            when_released_up: || return false,
            when_pressing_down: || return false,
            when_released_down: || return false,
            when_pressing_turn_left: || return false,
            when_released_turn_left: || return false,
            when_pressing_turn_right: || return false,
            when_released_turn_right: || return false,
            when_pressing_turn_up: || return false,
            when_released_turn_up: || return false,
            when_pressing_turn_down: || return false,
            when_released_turn_down: || return false
        };
    }
}

pub fn listen(mut key_listener: KeyListener) {
    // dont need this now
    // let _handle = thread::spawn(move || {
    //     loop {  }
    // });
    if key_listener.should_stop {
        return;
    }

    let input = getch();

    match input {
        32 => key_listener.is_holding_space = true,
        _ => println!("nothing")
    }

    // space
    if key_listener.is_holding_space {
        key_listener.when_pressing_space;
        key_listener.is_holding_space = false;
        key_listener.was_holding_space = true;
    } else {
        key_listener.when_released_space;
        key_listener.was_holding_space = false;
    }
}
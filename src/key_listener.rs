/*
    class to listen to the keyboard input
    uses ncurses and its own thread
    seems the best way
*/

use device_query::{DeviceQuery, DeviceState, Keycode}; // using this for now, for simplicity
use ncurses::mvprintw;
use crate::player::Player;

// #[derive(Clone)]
pub struct KeyListener {
    pub device_state: DeviceState,
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
    pub when_pressing_forward: fn(player: Player) -> Player,
    pub when_released_forward: fn(player: Player) -> Player,
    pub when_pressing_backward: fn(player: Player) -> Player,
    pub when_released_backward: fn(player: Player) -> Player,
    pub when_pressing_left: fn(player: Player) -> Player,
    pub when_released_left: fn(player: Player) -> Player,
    pub when_pressing_right: fn(player: Player) -> Player,
    pub when_released_right: fn(player: Player) -> Player,
    pub when_pressing_up: fn(player: Player) -> Player,
    pub when_released_up: fn(player: Player) -> Player,
    pub when_pressing_down: fn(player: Player) -> Player,
    pub when_released_down: fn(player: Player) -> Player,
    pub when_pressing_turn_left: fn(player: Player) -> Player,
    pub when_released_turn_left: fn(player: Player) -> Player,
    pub when_pressing_turn_right: fn(player: Player) -> Player,
    pub when_released_turn_right: fn(player: Player) -> Player,
    pub when_pressing_turn_up: fn(player: Player) -> Player,
    pub when_released_turn_up: fn(player: Player) -> Player,
    pub when_pressing_turn_down: fn(player: Player) -> Player,
    pub when_released_turn_down: fn(player: Player) -> Player
}

impl KeyListener {
    pub fn new() -> KeyListener{
        return KeyListener { ..Default::default() };
    }


    pub fn dismiss(&mut self) {
        self.should_stop = true;
    }
}

impl Default for KeyListener {
    fn default() -> Self {
        return KeyListener {
            device_state: DeviceState::new(),

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
            when_pressing_forward: |player| { player },
            when_released_forward: |player| { player },
            when_pressing_backward: |player| { player },
            when_released_backward: |player| { player },
            when_pressing_left: |player| { player },
            when_released_left: |player| { player },
            when_pressing_right: |player| { player },
            when_released_right: |player| { player },
            when_pressing_up: |player| { player },
            when_released_up: |player| { player },
            when_pressing_down: |player| { player },
            when_released_down: |player| { player },
            when_pressing_turn_left: |player| { player },
            when_released_turn_left: |player| { player },
            when_pressing_turn_right: |player| { player },
            when_released_turn_right: |player| { player },
            when_pressing_turn_up: |player| { player },
            when_released_turn_up: |player| { player },
            when_pressing_turn_down: |player| { player },
            when_released_turn_down: |player| { player }
        };
    }
}

pub fn listen(key_listener: &mut KeyListener, mut player: Player) -> Player {
    if key_listener.should_stop {
        return player;
    }

    let keys: Vec<Keycode> = key_listener.device_state.get_keys(); // how does this work ???
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

                _ => _ = mvprintw(i as i32+8, 0, &("key: ".to_owned()+&key.to_string()))
            }
        }
    }

    // forward
    if key_listener.is_holding_forward {
        mvprintw(0, 0, &("Holding forward".to_owned()));
        player = (key_listener.when_pressing_forward)(player);
        key_listener.was_holding_forward = true;
        key_listener.is_holding_forward = false;
    } else if key_listener.was_holding_forward && !key_listener.is_holding_forward { // when the user releases forward
        player = (key_listener.when_released_forward)(player);
        key_listener.was_holding_forward = false;
        mvprintw(0, 0, &("Released forward".to_owned()));
    }

    // backward
    if key_listener.is_holding_backward {
        mvprintw(0, 0, &("Holding backward".to_owned()));
        player = (key_listener.when_pressing_backward)(player);
        key_listener.was_holding_backward = true;
        key_listener.is_holding_backward = false;
    } else if key_listener.was_holding_backward && !key_listener.is_holding_backward { // when the user releases backward
        player = (key_listener.when_released_backward)(player);
        key_listener.was_holding_backward = false;
        mvprintw(0, 0, &("Released backward".to_owned()));
    }

    // left
    if key_listener.is_holding_left {
        mvprintw(0, 0, &("Holding left".to_owned()));
        player = (key_listener.when_pressing_left)(player);
        key_listener.was_holding_left = true;
        key_listener.is_holding_left = false;
    } else if key_listener.was_holding_left && !key_listener.is_holding_left { // when the user releases left
        player = (key_listener.when_released_left)(player);
        key_listener.was_holding_left = false;
        mvprintw(0, 0, &("Released left".to_owned()));
    }

    // right
    if key_listener.is_holding_right {
        mvprintw(0, 0, &("Holding right".to_owned()));
        player = (key_listener.when_pressing_right)(player);
        key_listener.was_holding_right = true;
        key_listener.is_holding_right = false;
    } else if key_listener.was_holding_right && !key_listener.is_holding_right { // when the user releases right
        player = (key_listener.when_released_right)(player);
        key_listener.was_holding_right = false;
        mvprintw(0, 0, &("Released right".to_owned()));
    }

    // up
    if key_listener.is_holding_up {
        mvprintw(0, 0, &("Holding up".to_owned()));
        player = (key_listener.when_pressing_up)(player);
        key_listener.was_holding_up = true;
        key_listener.is_holding_up = false;
    } else if key_listener.was_holding_up && !key_listener.is_holding_up { // when the user releases up
        player = (key_listener.when_released_up)(player);
        key_listener.was_holding_up = false;
        mvprintw(0, 0, &("Released up".to_owned()));
    }

    // down
    if key_listener.is_holding_down {
        mvprintw(0, 0, &("Holding down".to_owned()));
        player = (key_listener.when_pressing_down)(player);
        key_listener.was_holding_down = true;
        key_listener.is_holding_down = false;
    } else if key_listener.was_holding_down && !key_listener.is_holding_down { // when the user releases down
        player = (key_listener.when_released_down)(player);
        key_listener.was_holding_down = false;
        mvprintw(0, 0, &("Released down".to_owned()));
    }

    // turn_left
    if key_listener.is_holding_turn_left {
        mvprintw(0, 0, &("Holding turn_left".to_owned()));
        player = (key_listener.when_pressing_turn_left)(player);
        key_listener.was_holding_turn_left = true;
        key_listener.is_holding_turn_left = false;
    } else if key_listener.was_holding_turn_left && !key_listener.is_holding_turn_left { // when the user releases turn_left
        player = (key_listener.when_released_turn_left)(player);
        key_listener.was_holding_turn_left = false;
        mvprintw(0, 0, &("Released turn_left".to_owned()));
    }

    // turn_right
    if key_listener.is_holding_turn_right {
        mvprintw(0, 0, &("Holding turn_right".to_owned()));
        player = (key_listener.when_pressing_turn_right)(player);
        key_listener.was_holding_turn_right = true;
        key_listener.is_holding_turn_right = false;
    } else if key_listener.was_holding_turn_right && !key_listener.is_holding_turn_right { // when the user releases turn_right
        player = (key_listener.when_released_turn_right)(player);
        key_listener.was_holding_turn_right = false;
        mvprintw(0, 0, &("Released turn_right".to_owned()));
    }

    // turn_up
    if key_listener.is_holding_turn_up {
        mvprintw(0, 0, &("Holding turn_up".to_owned()));
        player = (key_listener.when_pressing_turn_up)(player);
        key_listener.was_holding_turn_up = true;
        key_listener.is_holding_turn_up = false;
    } else if key_listener.was_holding_turn_up && !key_listener.is_holding_turn_up { // when the user releases turn_up
        player = (key_listener.when_released_turn_up)(player);
        key_listener.was_holding_turn_up = false;
        mvprintw(0, 0, &("Released turn_up".to_owned()));
    }

    // turn_down
    if key_listener.is_holding_turn_down {
        mvprintw(0, 0, &("Holding turn_down".to_owned()));
        player = (key_listener.when_pressing_turn_down)(player);
        key_listener.was_holding_turn_down = true;
        key_listener.is_holding_turn_down = false;
    } else if key_listener.was_holding_turn_down && !key_listener.is_holding_turn_down { // when the user releases turn_down
        player = (key_listener.when_released_turn_down)(player);
        key_listener.was_holding_turn_down = false;
        mvprintw(0, 0, &("Released turn_down".to_owned()));
    }

    return player;
}
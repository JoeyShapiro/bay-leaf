stuff = [ 'forward', 'backward', 'left', 'right', 'up', 'down', 'turn_left', 'turn_right', 'turn_up', 'turn_down' ]

for thing in stuff:
    print(f'''
// {thing}
if key_listener.is_holding_{thing} {{
    mvprintw(0, 0, &("Holding {thing}".to_owned()));
    key_listener.when_pressing_{thing};
    key_listener.was_holding_{thing} = true;
    key_listener.is_holding_{thing} = false;
}} else if key_listener.was_holding_{thing} && !key_listener.is_holding_{thing} {{ // when the user releases {thing}
    key_listener.when_released_{thing};
    key_listener.was_holding_{thing} = false;
    mvprintw(0, 0, &("Released {thing}".to_owned()));
}}''')

'''
// up
        if key_listener.is_holding_space {
            mvprintw(0, 0, &("Holding Space".to_owned()));
            key_listener.when_pressing_space;
            key_listener.was_holding_space = true;
        } else if key_listener.was_holding_space && key_listener.is_holding_space { // when the user releases up
            key_listener.when_released_space;
            key_listener.is_holding_space = false;
            key_listener.was_holding_space = false;
        }
'''
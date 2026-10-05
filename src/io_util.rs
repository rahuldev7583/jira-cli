use std::io;

pub fn get_user_input() -> String {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer);

    // println!("buffer: {}", buffer);

    buffer
}

pub fn wait_for_key_press() {
    io::stdin().read_line(&mut String::new()).unwrap();
}

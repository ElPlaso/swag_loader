use std::{
    io::{self, Write},
    thread, time,
};

fn main() {
    print!("Loading Swag...\n");
    for i in 0..17 {
        print!("\r{}", std::iter::repeat("█").take(i).collect::<String>());
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(200));
    }
    print!(" 100% Complete.")
}

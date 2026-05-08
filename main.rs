use std::{thread, time};

fn main() {
    loop {
        println!("LED ON");
        thread::sleep(time::Duration::from_millis(500));

        println!("LED OFF");
        thread::sleep(time::Duration::from_millis(500));
    }
}
use rustybox::Function;
use std::env::Args;

pub fn binding() -> (&'static str, Function) {
    ("yes", Box::new(yes))
}

fn yes(_: Args) -> i32 {
    loop {
        println!("y")
    }
    0
}

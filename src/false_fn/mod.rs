use std::env::Args;
use rustybox::Function;

pub fn binding() -> (&'static str, Function) {
    ("false", Box::new(false_fn))
}

fn false_fn(_: Args) -> i32 {
    1
}

use std::process::exit;
use std::collections::HashMap;

mod rustybox;
mod cat;

use rustybox::RBox;

use cat::cat;

fn main() {
    let mut rbox = RBox::new();

    rbox.add_func("cat", Box::new(cat));

    let functionMap: HashMap<String, i32> = HashMap::new();
    let mut args = std::env::args();
    let mut invocation_path = args.next().unwrap();
    let mut invocation_name = invocation_path.split('/').last().unwrap();
    match invocation_name {
        "rustybox" => {
            if let Some(next_name) = args.next() {
                exit(rbox.call(next_name, args));
            } else {
                exit(rbox.usage())
            }
        },
        _ => {
            exit(rbox.call(invocation_name, args));
        }
    }
}

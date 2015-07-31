use std::process::exit;

mod rustybox;
mod cat;
mod false_fn;
mod yes;

use rustybox::RBox;

fn main() {
    let mut rbox = RBox::new();

    rbox.add_func(cat::binding());
    rbox.add_func(false_fn::binding());
    rbox.add_func(yes::binding());

    let mut args = std::env::args();
    let invocation_path = args.next().unwrap();
    let invocation_name = invocation_path.split('/').last().unwrap();
    exit(match invocation_name {
            "rustybox" => {
                if let Some(next_name) = args.next() {
                    rbox.call(next_name, args)
                } else {
                    rbox.usage()
                }
            },
            _ => {
                rbox.call(invocation_name, args)
            }
        });
}

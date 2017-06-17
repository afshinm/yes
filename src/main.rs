use std::env;

#[allow(while_true)]
fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed
        1 => {
            while true {
                println!("{}", "y");
            }
        },
        _ => {
            let s: String = args[1..].join("");

            while true {
                println!("{}", s);
            }
        }
    }
}

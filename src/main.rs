use std::env;

fn create_buffer(size: usize) -> String {
    String::with_capacity(size * 4096)
}

#[allow(while_true)]
fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed
        1 => {
            // 2 because of "y\n"
            let mut s_buffer = create_buffer(2);

            for _ in 0..4096 {
                s_buffer.push_str("y\n");
            }

            while true {
                print!("{}", s_buffer);
            }
        },
        _ => {
            let s: String = args[1..].join("");

            let mut s_buffer = create_buffer(s.len());

            for _ in 0..4096 {
                s_buffer.push_str(&s);
            }

            while true {
                println!("{}", s_buffer);
            }
        }
    }
}

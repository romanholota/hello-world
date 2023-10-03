use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    let name = match args.get(1) {
        Some(val) => val,
        None => "world"
    };

    println!("Hello, {}!", name);

}

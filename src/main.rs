const VERSION: &str = env!("CARGO_PKG_VERSION");
mod utils;
fn main() {
    println!("Hello, world!");
    let args: Vec<String> = std::env::args().collect();
    process_args(args);
}

fn process_args(mut args: Vec<String>) {
    match args.remove(0).as_str() {
        "--version" => {
            println!("Version: {}", VERSION);
        }
        "echo" => {
            utils::echo::main(args);
        }
        a => {
            println!("Invalid subcommand or first argument: \"{}\"", a);
        }
    }
}

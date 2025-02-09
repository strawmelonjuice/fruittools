const VERSION: &str = env!("CARGO_PKG_VERSION");
mod utils;
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
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
        "changelog" | "bananen" | "banana" => {
            utils::bananen(args);
        }
        a => {
            println!("Invalid subcommand or first argument: \"{}\"", a);
        }
    }
}

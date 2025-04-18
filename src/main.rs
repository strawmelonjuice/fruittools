const VERSION: &str = env!("CARGO_PKG_VERSION");
mod utils;
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    process_args(args);
}

fn process_args(mut args: Vec<String>) {
    if args.len() <= 0 {
        args = vec![String::from("help")]
    }
    match args.remove(0).to_lowercase().as_str() {
        "--version" => {
            println!("Version: {}", VERSION);
        }
        "echo" => {
            utils::echo::main(args);
        }
        "changelog" | "bananen" | "banana" => utils::bananen(args),
        "run" | "pulp" => utils::pulp(args),
        "help" => {
            todo!("Print some help here!");
        }
        a => {
            println!("Invalid subcommand or first argument: \"{}\"", a);
        }
    }
}

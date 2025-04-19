const VERSION: &str = env!("CARGO_PKG_VERSION");
mod utils;
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    process_args(args);
}

fn process_args(mut args: Vec<String>) {
    if args.is_empty() {
        args = vec![String::from("help")]
    }
    match args.remove(0).to_lowercase().as_str() {
        "--version" => {
            println!("Version: {}", VERSION);
        }
        "echo" | "say" | "print" | "tell" => utils::echo::main(args),
        #[cfg(feature = "tool-bananen")]
        "changelog" | "bananen" | "banana" => utils::bananen(args),
        #[cfg(feature = "tool-pulp")]
        "run" | "runner" | "pulp" => utils::pulp(args),
        "help" => {
            todo!("Print some help here!");
        }
        a => {
            println!("Invalid subcommand or first argument: \"{}\"", a);
        }
    }
}

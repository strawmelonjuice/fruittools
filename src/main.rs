use cynthia_con::CynthiaStyles;
use cynthia_con::CynthiaColors;
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
        "help" => {println!("{} Displaying help\n", "🚑".style_bold().color_bright_red());
            println!("\t{}", "Commands:".style_bold().style_underline());
            println!("\tI'll be adding more commands soon! 🚀");}
        a => {
            println!("Invalid subcommand or first argument: \"{}\"", a);
        }
    }
}

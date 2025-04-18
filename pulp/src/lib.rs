use cynthia_con::{CynthiaColors, CynthiaStyles};
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn interpret(mut args: Vec<String>) {
    println!(
        "{}{}\nBy {}\n{}",
        cynthia_con::horizline(),
        format!("{} v{VERSION}", "Pulp! 🍹".color_lilac())
            .style_bold()
            .style_centered(),
        format!(
            "{}{}{}",
            "Straw".color_red(),
            "melon".color_green(),
            "juice".color_orange()
        ),
        cynthia_con::horizline()
    );
    if args.is_empty() {
        eprintln!(
            "{}{}",
            "No arguments provided.".color_red(),
            " Please provide some arguments.".color_yellow()
        );
        return;
    }
    match args.remove(0).as_str() {
        "help" | "man" => {
            println!("{} Displaying help\n", "🚑".style_bold().color_bright_red());
            println!("\t{}", "Commands:".style_bold().style_underline());
            println!("\tI'll be adding more commands soon! 🚀");
        }
        "version" => {
            println!(
                "{}",
                format!("Version: {}", VERSION).color_green().style_bold()
            );
        }
        _ => {
            eprintln!(
                "{}",
                "Invalid command. Please use 'help' for more information."
                    .color_red()
                    .style_bold()
            );
        }
    }
}

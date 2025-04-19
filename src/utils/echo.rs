pub fn main(args: Vec<String>) {
    let mut sen = String::new();
    for arg in args {
        sen.push_str(format!(" {}", arg).as_str());
    }
    println!("{}", sen.trim());
}

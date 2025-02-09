pub fn main(args: Vec<String>) {
    let mut sen = String::new();
    let mut i: usize = 0;
    for arg in args {
        if i == 0 {
            i = i + 1;
            continue;
        }
        sen.push_str(format!(" {}", arg).as_str());
        i = i + 1;
    }
    println!("{}", sen.trim());
}

pub(crate) fn specific_release(args: Vec<String>) -> Option<String> {
    if args.is_empty() {
        return None;
    } else {
        let mut out = None;
        for arg in args {
            if arg.starts_with("--release=") {
                out = Some(arg.split_at(10).1.to_string());
                break;
            }
        }
        out
    }
}

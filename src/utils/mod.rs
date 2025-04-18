pub(crate) fn pulp(args: Vec<String>) {
    pulp::interpret(args);
}
pub(crate) fn bananen(args: Vec<String>) {
    bananen::interpret(args);
}

pub(crate) mod echo;

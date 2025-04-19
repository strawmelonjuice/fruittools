pub(crate) mod echo;
#[cfg(feature = "tool-pulp")]
pub(crate) fn pulp(args: Vec<String>) {
    pulp::interpret(args);
}
#[cfg(feature = "tool-bananen")]
pub(crate) fn bananen(args: Vec<String>) {
    bananen::interpret(args);
}

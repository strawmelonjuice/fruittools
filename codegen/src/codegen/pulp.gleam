import gleam/dynamic/decode

pub fn main() {
  todo
}

/// Compiles the TruthSource at ./pulp/pulpets.toml into a single rust function that returns
/// a simple List of items (or preferably be one option)
///
/// So that pulp can query which item to run and wether to save this to the database.
fn items() {
  let doc_comment = ["in"]
  todo
}

/// As a basic principle, the ones with a *number* in their doc comment, just count up.
/// If there's a clear winner in number of points, that one will definitely be ran, otherwise a picker will be shown.
type PulpTruthSourceItem {
  PulpTruthSourceItem(
    name: String,
    filesfound: List(String),
    /// *1* Trigger this item if a substring is found in one of the
    foundinfiles: List(#(String, String)),
    /// *2* In which mode to use this item preferably
    preferred_in_mode: Mode,
    /// Regex of other items to override.
    /// For example, "*", because having a mise.toml with a misetask called 'pulp', 'pulp-watch' or 'pulp-run'
    /// is pretty self-explanatory. However if none of those exist in the mise.toml we'd have to ask if any of the other options is ranked higher.
    overides: String,
    overides_in_mode: List(Mode),
    command: String,
    args: List(String),
  )
}

fn pulp_truth_source_item_decoder() -> decode.Decoder(PulpTruthSourceItem) {
  use name <- decode.field("name", decode.string)
  use filesfound <- decode.field("filesfound", decode.list(decode.string))
  use foundinfiles <- decode.field(
    "foundinfiles",
    decode.list({
      use a <- decode.field(0, decode.string)
      use b <- decode.field(1, decode.string)

      decode.success(#(a, b))
    }),
  )
  use preferred_in_mode <- decode.field("preferred_in_mode", mode_decoder())
  use overides <- decode.field("overides", decode.string)
  use overides_in_mode <- decode.field(
    "overides_in_mode",
    decode.list(mode_decoder()),
  )
  use command <- decode.field("command", decode.string)
  use args <- decode.field("args", decode.list(decode.string))
  decode.success(PulpTruthSourceItem(
    name:,
    filesfound:,
    foundinfiles:,
    preferred_in_mode:,
    overides:,
    overides_in_mode:,
    command:,
    args:,
  ))
}

type Mode {
  Asked
  Watch
  Run
  Test
}

fn mode_decoder() -> decode.Decoder(Mode) {
  use variant <- decode.then(decode.string)
  case variant {
    "asked" -> decode.success(Asked)
    "watch" -> decode.success(Watch)
    "run" -> decode.success(Run)
    "test" -> decode.success(Test)
    _ -> decode.failure(todo as "Zero value for Mode", "Mode")
  }
}

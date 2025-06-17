import gleam/bool
import gleam/int
import gleam/list
import gleam/string

pub const max_comment_length = 110

pub const max_comment_length_tolerance = 5

pub fn printable(in: List(String)) {
  in
  |> line_breaks
  |> break_up
  |> list.map(fn(p) { "[codegen >] " <> p })
  |> string.join("\n")
  <> "\n"
}

pub fn comments(in: List(String)) {
  in
  |> line_breaks
  |> break_up
  |> list.map(fn(p) { "// " <> p })
  |> string.join("\n")
}

pub fn doc_comments(in: List(String)) {
  in
  |> line_breaks
  |> break_up
  |> list.map(fn(p) { "/// " <> p })
  |> string.join("\n")
}

fn break_up(strings_to_go: List(String)) {
  let #(again, out) =
    list.map_fold(
      strings_to_go,
      from: True,
      with: fn(last_one_too_long, old_input_string) {
        let too_long = string.length(old_input_string) > max_comment_length
        let tolerant_too_long =
          string.length(old_input_string)
          > int.add(max_comment_length, max_comment_length_tolerance)
        let new_strings = case too_long {
          True -> {
            let is_broken = {
              let a =
                string.slice(
                  from: old_input_string,
                  at_index: int.subtract(max_comment_length, 1),
                  length: 2,
                )
              let b = string.trim(string.replace(a, " ", ""))
              // if a==b that means there's no whitespace in a. No whitespace on the break means there's a word being broken.
              a == b
            }
            let one =
              string.slice(
                from: old_input_string,
                at_index: 0,
                length: max_comment_length,
              )

            let two = string.drop_start(old_input_string, max_comment_length)
            case is_broken, tolerant_too_long {
              True, False -> [old_input_string]
              False, _ -> [one, two]
              // If not even tolerance can save it, at least add a hyphen...
              True, True -> [one, "-" <> two]
            }
          }
          False -> [old_input_string]
        }
        // If it was too long, better be safe than sorry. Let's run it once more.
        let any_too_long = case last_one_too_long {
          True -> True
          False -> tolerant_too_long
        }
        #(any_too_long, new_strings)
      },
    )
  let output = list.flatten(out)
  use <- bool.guard(again, output)
  break_up(output)
}

fn line_breaks(in: List(String)) {
  string.join(in, "\n")
  |> string.split("\n")
}

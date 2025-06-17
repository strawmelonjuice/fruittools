import codegen/internal/comments
import gleam/int
import gleam/string
import gleamy_lights/premixed
import gleamy_lights/premixed/gleam_colours
import gleamyshell
import simplifile

pub fn infoblock() {
  // Collect variables
  let assert Ok(cwd) = simplifile.current_directory()
  let commit = case
    gleamyshell.execute("git", in: ".", args: ["rev-parse", "--short", "HEAD"])
  {
    Ok(gleamyshell.CommandOutput(0, rev)) -> {
      rev |> string.trim
    }
    _ -> "unknown"
  }
  let comments = int.to_string(comments.max_comment_length)
  let comments_tol = int.to_string(comments.max_comment_length_tolerance)

  // Colour in the labels
  let label_cwd = premixed.text_bright_orange("cwd:")
  let label_commit = premixed.text_bright_orange("from commit:")
  let label_comments = premixed.text_bright_orange("max comment length:")
  let label_comments_tol = premixed.text_bright_orange("with tolerance:")

  // Into a printable!
  comments.printable([
    "\t\t  running in" |> gleam_colours.text_aged_plastic_yellow,
    "",
    label_cwd <> "\t\t" <> cwd,
    label_commit <> "\t\t" <> commit,
    "",
    "\t\t generated comments:" |> gleam_colours.text_aged_plastic_yellow,
    "",
    label_comments <> "\t" <> comments,
    label_comments_tol <> "\t" <> comments_tol,
  ])
}

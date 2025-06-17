import birdie
import codegen/internal/comments
import gleeunit

pub fn main() {
  gleeunit.main()
}

pub fn single_doc_comment_test() {
  comments.doc_comments(["Single doc comment"])
  |> birdie.snap(title: "single doc comment generation")
}

pub fn loads_of_doc_comments_test() {
  comments.doc_comments([
    "First doc comment", "", "A really long doc comment:",
    "iodj hauigr hbuhguy kshuyr ewhayuj yugrgbay yfdgyfsufgys difjodf fjfijwo aiojdaoidj aidjadoo adijdiojdoja aido ada  dadad asjisj ad",
    "And so it does:",
    "ddioshfu eufhsihfs shfuhsfhuf sififi sisis udihsf uisfhiuf ushfufshf suhfifhui usfuhuf dauhd hhh dhhhd auh broken adadada daf fadfgh",
    "But then, no spaces!", "",
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  ])
  |> birdie.snap(title: "long and multiple doc comment generation")
}

pub fn single_comment_test() {
  comments.comments(["Single comment"])
  |> birdie.snap(title: "single comment generation")
}

pub fn loads_of_comments_test() {
  comments.comments([
    "First comment", "", "A really long comment:",
    "iodj hauigr hbuhguy kshuyr ewhayuj yugrgbay yfdgyfsufgys difjodf fjfijwo aiojdaoidj aidjadoo adijdiojdoja aido ada  dadad asjisj ad",
    "And so it does:",
    "ddioshfu eufhsihfs shfuhsfhuf sififi sisis udihsf uisfhiuf ushfufshf suhfifhui usfuhuf dauhd hhh dhhhd auh broken adadada daf fadfgh",
    "But then, no spaces!", "",
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  ])
  |> birdie.snap(title: "long and multiple comment generation")
}

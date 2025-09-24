import gleam/io
import gleam/string
import gleamy_lights
import gleamy_lights/premixed
import plinth/javascript/global

pub fn main() -> Nil {
  opening_animation()
}

fn opening_animation() {
  let timeout_nilled = fn(delay: Int, cb: fn() -> a) -> Nil {
    global.set_timeout(delay, cb)
    Nil
  }
  // Animation speed.
  let to = 80
  let strawmelonjuice =
    premixed.text_lime("straw")
    <> premixed.text_red("melon")
    <> premixed.text_yellow("juice")
    <> " " |> string.repeat(5)
  // Totals 35 characters including 'strawmelonjuice'-colours.
  io.print("🍇🍉🍌🍒tools! By " <> strawmelonjuice)
  use <- timeout_nilled(to)
  io.print("\u{8}" |> string.repeat(38))
  // f
  let fruit_composed = premixed.text_purple("f")
  io.print(fruit_composed <> "🍉🍌🍒tools! By " <> strawmelonjuice)
  use <- timeout_nilled(to)
  io.print("\r")
  // fr
  let fruit_composed = fruit_composed <> premixed.text_pink("r")
  io.print(fruit_composed <> "🍌🍒tools! By " <> strawmelonjuice)
  use <- timeout_nilled(to)
  io.print("\u{8}" |> string.repeat(40))
  // fru
  let fruit_composed = fruit_composed <> premixed.text_orange("u")
  io.print(fruit_composed <> "🍒tools! By " <> strawmelonjuice)
  use <- timeout_nilled(to)
  io.print("\u{8}" |> string.repeat(42))
  // frui
  let fruit_composed = fruit_composed <> premixed.text_yellow("i")
  io.print(fruit_composed <> "🍒tools! By " <> strawmelonjuice)
  use <- timeout_nilled(to)
  io.print("\u{8}" |> string.repeat(44))
  // fruit
  let fruit_composed = fruit_composed <> premixed.text_green("t")
  io.print(fruit_composed <> "tools! By " <> strawmelonjuice)
  use <- timeout_nilled(to)
  io.print("\u{8}" |> string.repeat(46))
  // fruit in orange
  let fruit_composed = premixed.text_orange("fruit")
  io.print(fruit_composed <> "tools! By " <> strawmelonjuice)
  use <- timeout_nilled(to)
  io.print("\u{8}" |> string.repeat(48))
  // tools in cyan
  io.print({
    premixed.text_lime("fruit")
    <> premixed.text_orange("tools")
    <> "! By "
    <> gleamy_lights.by_rgb("straw", 3, 125, 80)
    // |> premixed.bg_white()
    <> premixed.text_bright_red("melon")
    <> gleamy_lights.by_rgb("juice", 245, 119, 63)
    // <> " " |> string.repeat(5)
  })
  use <- timeout_nilled(50)
  io.print(". ")
  use <- timeout_nilled(20)
  io.println(premixed.bg_yellow(premixed.text_black(":)")))
}

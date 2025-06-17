import codegen/internal
import codegen/pulp
import gleam/io

pub fn main() -> Nil {
  io.print(internal.infoblock())
  let _ = pulp.main()
}

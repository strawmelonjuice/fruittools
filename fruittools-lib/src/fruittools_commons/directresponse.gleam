import gleam/bool
import gleam/dynamic/decode
import gleam/json
import gleam/option.{type Option, None, Some}

/// Represents the
pub type DirectResponse {
  DirectResponse(status: Int, directoutput: Option(String))
}

pub fn direct_response_to_json(direct_response: DirectResponse) -> json.Json {
  let DirectResponse(status:, directoutput:) = direct_response
  case directoutput {
    Some(direct_output) -> {
      json.object([
        #("status", json.int(status)),
        #("with_direct_output", json.bool(True)),
        #("direct_output", json.string(direct_output)),
      ])
    }
    None -> {
      json.object([
        #("status", json.int(status)),
        #("with_direct_output", json.bool(False)),
      ])
    }
  }
}

pub fn direct_response_decoder() -> decode.Decoder(DirectResponse) {
  use status <- decode.field("status", decode.int)
  use with_direct_output <- decode.field("with_direct_output", decode.bool)
  use <- bool.guard(
    with_direct_output,
    decode.success(DirectResponse(status:, directoutput: None)),
  )
  use direct_output <- decode.field("direct_output", decode.string)
  decode.success(DirectResponse(status:, directoutput: Some(direct_output)))
}

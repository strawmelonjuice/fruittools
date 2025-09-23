import gleam/dynamic/decode
import gleam/json

pub type DirectResponse {
  DirectResponse(status: Int, directoutput: String)
}

pub fn direct_response_to_json(direct_response: DirectResponse) -> json.Json {
  let DirectResponse(status:, directoutput:) = direct_response
  json.object([
    #("status", json.int(status)),
    #("directoutput", json.string(directoutput)),
  ])
}

pub fn direct_response_decoder() -> decode.Decoder(DirectResponse) {
  use status <- decode.field("status", decode.int)
  use directoutput <- decode.field("directoutput", decode.string)
  decode.success(DirectResponse(status:, directoutput:))
}

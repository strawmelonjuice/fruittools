import gleam/int
import gleam/result
import gmsg

pub type DirectResponse {
  DirectResponse(status: Int, directoutput: String)
}

pub fn encode_direct_response(
  response: DirectResponse,
) -> Result(BitArray, EncodeError) {
  MsgMap([
    #(
      gmsg.MsgString("status"),
      gmsg.MsgString(response.status |> int.to_string()),
    ),
    #(gmsg.MsgString("directoutput"), gmsg.MsgString(response.directoutput)),
  ])
}

pub fn decode_direct_response(
  from bits: BitArray,
) -> Result(DirectResponse, gmsg.DecodeError) {
  let pair_list_decoder = dict_decoder |> decode.map(dict_to_pairs)
  let out: Result(List(#(String, String)), DecodeError) =
    gmsg.parse(bits, pair_list_decoder)
    |> result.map_error(DecodeErrorA)
  use out <- result.try(out)
  todo
}

pub type DecodeError {
  DecodeErrorA(gmsg.DecodeError)
  FieldError
}

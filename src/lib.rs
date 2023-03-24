use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn receive_bytes(file: &mut [u8]) -> Result<(), JsValue> {
    Ok(())
}

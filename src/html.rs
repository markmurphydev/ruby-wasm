use std::fs;
use base64::Engine;

/// Write an HTML wrapper that runs the given .wasm file
/// - `wasm_bytes`: `.wasm` file to embed, as a slice of bytes.
pub fn make_html_wrapper(wasm_bytes: &[u8]) -> String {
    let template = include_str!("../resources/wasm_runner.template.html");
    let wasm_base64 = base64::prelude::BASE64_STANDARD.encode(wasm_bytes);
    let html = template.replace("%{wasm_base64}", &wasm_base64);
    html
}
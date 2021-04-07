use wasm_bindgen::prelude::*;

// wasm-bindgen will automatically take care of including this script
#[wasm_bindgen(module = "/src/openFileDialog.js")]
extern "C" {
    #[wasm_bindgen(js_name = "openPK8File")]
    pub fn open_pk8_file(payload_callback: JsValue);
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "console"])]
    pub fn log(stuff: JsValue);

    #[wasm_bindgen(js_namespace = ["window", "Date"])]
    pub fn now() -> f64;
}

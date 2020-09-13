use wasm_bindgen_test::wasm_bindgen_test;
use web_sugars::prelude as ws;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn get_window()
{
 ws::get_window().unwrap();
}

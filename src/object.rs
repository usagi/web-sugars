use js_sys as js;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "export Object")]
extern {
 pub type Object;

 #[wasm_bindgen(constructor)]
 pub fn new() -> Object;
}

impl Object
{
 pub fn set(o: &JsValue, key: &str, value: &JsValue) -> Result<bool, JsValue>
 {
  #[allow(unused_unsafe)]
  unsafe {
   js::Reflect::set(o, &key.into(), value)
  }
 }

 pub fn get(o: &JsValue, key: &str) -> Result<JsValue, JsValue>
 {
  #[allow(unused_unsafe)]
  unsafe {
   js::Reflect::get(o, &key.into())
  }
 }

 pub fn get_or_new_object(o: &JsValue, key: &str) -> Result<JsValue, JsValue>
 {
  match Object::get(o, key)
  {
   Ok(p) if p.is_object() => Ok(p),
   _ =>
   {
    Object::set(o, key, &js::Object::new())?;
    Object::get(o, key)
   }
  }
 }
}

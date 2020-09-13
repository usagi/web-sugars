use wasm_bindgen::JsValue;

pub trait AsStringOr<T>
where
 T: AsRef<str>
{
 /// .as_string + unwrap_or
 /// - default return this value if failed.
 fn as_string_or(&self, default: T) -> String;
}

impl AsStringOr<String> for JsValue
{
 fn as_string_or(&self, default: String) -> String
 {
  self.as_string().unwrap_or(default)
 }
}

impl AsStringOr<&str> for JsValue
{
 fn as_string_or(&self, default: &str) -> String
 {
  self.as_string().unwrap_or(String::from(default))
 }
}


/// Add the JsValue constructor of the Object type.; `{}`
pub trait ConstructObject
{
 /// Construcct a JsValue of the Object type.; `{}`
 fn object() -> JsValue;
}

impl ConstructObject for JsValue
{
 fn object() -> JsValue
 {
  let o = crate::object::Object::new();
  JsValue::from(o)
 }
}


// #[wasm_bindgen]
// pub async fn array_buffer_to_vec(v: JsValue) -> Result<JsValue, JsValue>
// {
//  debug!("aaa1");
//  let a = v.as_uint8array().to_vec_sliced(2, 4);
//  for i in a.iter()
//  {
//   debug!("{}", i);
//  }
//  debug!("aaa5");
//  Ok(JsValue::null())
// }

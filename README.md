# web-sugars

It's a sugars for web related crates such as `web-sys`, `js-sys`, `wasm-bindgen`s.

## Targets of this crate

- Provide a useful syntax sugars.
- Provide a useful error handlings.

## Non-targets

- An Application specific features.
- An JS library specific features.

## Example

Easy error handling for function of a `#[wasm-bindgen]`, with easy common function calls:

```rust
use web_sugars::prelude::*;

#[wasm_bindgen]
pub fn my_wasm_func() -> Result<JsValue, JsValue> // <- by wasm-bindgen spec.
{
 // web-sugars functions are returns Result<T, WebSugarError>,
 // and WebSugarError has a From trait. Thus, you can type just `?` to throw an error.
 let _window = get_window()?;
 // No need `get_window()?.get_document()?` chains, just call `get_document`.
 let _document = get_document()?;
 // No need `.get_window()?.get_document()?.get_element_by_id()?` chains, just acll `get_element_by_id`.
 let _element = get_element_by_id("target-id")?;
 let _elements = get_elements_by_tag_name("div")?;
 let _xpath_results = evaluate("/html//p")?;
 // Fetch a String data, all of errors are handled by `WebSugarError`.
 let _data: String = fetch_as_string("https://example.com/my_data.txt")?;
 // Fetch a JSON
 let _data: JsValue = fetch_as_string("https://example.com/my_data.json")?;
 // Fetch a JSON, and then deserialize to `MyType` with serde.
 let _data: MyType = fetch_as_json_as::<MyType>("https://example.com/my_data.json")?;
 // Fetch a binary data to `JsValue` of `ArrayBuffer`
 let _data: JsValue = fetch_as_array-buffer("https://example.com/my_data.bin")?;
 // Get a Vec<T> from JsValue of ArrayBuffer easily
 let _data: Vec<u8> = _data.as_vec()?;

 // And more sugars may help you :)
}
```

## License

- [MIT](LICENSE.md)

## Author

- [USAGI.NETWORK / Usagi Ito](https://github.com/usagi)

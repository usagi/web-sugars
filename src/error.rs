use thiserror::Error;
use wasm_bindgen::JsValue;

/// Note: This type would be convert to JsValue via From trait.
/// Thus, you can code:
/// ```rust
/// use web_sys_sugars::prelude::get_window;
/// #[wasm_bindgen]
/// fn my_function() -> Result<JsValue, JsValue>
/// {
///  let _w = get_window()?;
///  // -> Resulut<Window, WebSysSugarsError>
///  //  Ok => Will be bound Window object to the `_w`.
///  //  Err(e: WebSysSugarsError) => implicit, JsValue::From(e) -> Err(JsValue)
///
///  // FYI, if you do not use this sugar crate, you will be need to write a long typing such as:
///  let _w = web_sys::window().ok_or(JsValue::from_str("Could not retrieve the Window object."))?;
/// }
/// ```
#[derive(Debug, Error)]
pub enum WebSysSugarsError
{
 #[error("Could not retrieve the Window object.")]
 CouldNotRetrieveWindow,
 #[error("Could not retrieve the Document object.")]
 CouldNotRetrieveDocument,
 #[error("Could not retrieve the Body element of the Document element.")]
 CouldNotRetrieveDocumentBody,
 #[error("Element ID {0} is not found.")]
 ElementIdIsNotFound(String),
 #[error("document.evaluate was failed: {0:?}")]
 EvaluateWasFailed(JsValue),
 #[error("requestAnimationFrame was failed: {0:?}")]
 RequestAnimationFrameWasFailed(JsValue),
 #[error("cancelAnimationFrame was failed: {0:?}")]
 CancelAnimationFrameWasFailed(JsValue),
 #[error("HTTP Response, status is not OK: Code = {0:?}")]
 HttpResponseStatusError(u16),
 #[error("HTTP Request, construction was failed: {0:?}")]
 HttpRequestConstructionWasFailed(JsValue),
 #[error("HTTP Request, await JsFuture error: {0:?}")]
 HttpRequestAwaitJsFutureError(JsValue),
 #[error("HTTP Request, got an unknown data that it not a Response type: unknown data = {0:?}")]
 HttpRequestResultIsNotResponseType(JsValue),
 #[error("HTTP Request, got a Response but .dyn_into was failed: response = {0:?}")]
 HttpResuestResultTypeIsResponseButDynIntoWasFailed(JsValue),
 #[error("HTTP Response, get .text() Promise was failed: {0:?}")]
 HttpResponseGetTextPromiseWasFailed(JsValue),
 #[error("HTTP Response, get .text() Future .await was failed: {0:?}")]
 HttpResponseGetTextFutureAwaitWasFailed(JsValue),
 #[error("HTTP Response, get .text() .as_string() was failed: {0:?}")]
 HttpResponseGetTextAsStringWasFailed(JsValue),
 #[error("HTTP Response, get .json() Promise was failed: {0:?}")]
 HttpResponseGetJsonPromiseWasFailed(JsValue),
 #[error("HTTP Response, get .json() Future .await was failed: {0:?}")]
 HttpResponseGetJsonFutureAwaitWasFailed(JsValue),
 #[error("HTTP Response, get .json() as T with serde was failed: {0:?}")]
 HttpResponseGetJsonAsSerdeError(serde_json::error::Error),
 #[error("HTTP Response, get .array_buffer() Promise was failed: {0:?}")]
 HttpResponseGetArrayBufferPromiseWasFailed(JsValue),
 #[error("HTTP Response, get .array_buffer() Future .await was failed: {0:?}")]
 HttpResponseGetArrayBufferFutureAwaitWasFailed(JsValue),
 #[error("Not implemented.")]
 NotImplemented
}

impl From<WebSysSugarsError> for JsValue
{
 fn from(v: WebSysSugarsError) -> Self
 {
  JsValue::from_str(&format!("{:?}", v)[..])
 }
}

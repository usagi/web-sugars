use crate::{
 error::WebSysSugarsError as E,
 response::AsHTTPResponseStatusResult,
 window::get_window
};

use log::*;
use serde::Deserialize;
use wasm_bindgen::{
 prelude::*,
 JsCast
};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
 Request,
 RequestInit,
 RequestMode,
 Response
};

pub async fn fetch(url: &str) -> Result<Response, E>
{
 trace!("fetch begin");
 let mut init = RequestInit::new();
 init.method("GET");
 init.mode(RequestMode::Cors);

 let request = Request::new_with_str_and_init(&url, &init).map_err(|e| E::HttpRequestConstructionWasFailed(e))?;

 let window = get_window()?;

 let response_jscast = JsFuture::from(window.fetch_with_request(&request))
  .await
  .map_err(|e| E::HttpRequestAwaitJsFutureError(e))?;

 let r = match response_jscast.is_instance_of::<Response>()
 {
  true =>
  {
   response_jscast
    .dyn_into()
    .map_err(|e| E::HttpResuestResultTypeIsResponseButDynIntoWasFailed(e))
  },
  false => Err(E::HttpRequestResultIsNotResponseType(response_jscast))
 };
 trace!("fetch to-exit");
 r
}

pub async fn fetch_as_string(url: &str) -> Result<String, E>
{
 trace!("fetch_as_string begin");
 let response: Response = fetch(url).await?;
 response.as_http_response_status_result()?;

 let r: JsValue = JsFuture::from(response.text().map_err(|e| E::HttpResponseGetTextPromiseWasFailed(e))?)
  .await
  .map_err(|e| E::HttpResponseGetTextFutureAwaitWasFailed(e))?;
 let r = r.as_string().ok_or(E::HttpResponseGetTextAsStringWasFailed(r));

 trace!("fetch_as_string to-exit");
 r
}

pub async fn fetch_as_json(url: &str) -> Result<JsValue, E>
{
 trace!("fetch_as_string begin");
 let response = fetch(url).await?;
 response.as_http_response_status_result()?;

 let r = JsFuture::from(response.json().map_err(|e| E::HttpResponseGetJsonPromiseWasFailed(e))?)
  .await
  .map_err(|e| E::HttpResponseGetJsonFutureAwaitWasFailed(e));

 trace!("fetch_as_string to-exit");
 r
}

/// Fetch -> JSON -> T; with serde
pub async fn fetch_as_json_as<T>(url: &str) -> Result<T, E>
where
 T: for<'a> Deserialize<'a>
{
 let json = fetch_as_json(url).await?;
 (json.into_serde() as Result<T, serde_json::error::Error>).map_err(|e| E::HttpResponseGetJsonAsSerdeError(e))
}

pub async fn fetch_as_array_buffer(url: &str) -> Result<JsValue, E>
{
 trace!("fetch_as_string begin");
 let response = fetch(url).await?;
 response.as_http_response_status_result()?;

 let r = JsFuture::from(
  response
   .array_buffer()
   .map_err(|e| E::HttpResponseGetArrayBufferPromiseWasFailed(e))?
 )
 .await
 .map_err(|e| E::HttpResponseGetArrayBufferFutureAwaitWasFailed(e));

 trace!("fetch_as_string to-exit");
 r
}

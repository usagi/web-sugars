use crate::error::WebSysSugarsError as E;
use web_sys::Response;

pub trait AsHTTPResponseStatusResult
{
 fn as_http_response_status_result(&self) -> Result<u16, E>;
}

impl AsHTTPResponseStatusResult for Response
{
 fn as_http_response_status_result(&self) -> Result<u16, E>
 {
  match self.status()
  {
   s if s >= 100 && s < 300 => Ok(s),
   s => Err(E::HttpResponseStatusError(s))
  }
 }
}

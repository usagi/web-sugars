use crate::error::WebSysSugarsError as E;
use std::{
 cell::RefCell,
 rc::Rc
};
use wasm_bindgen::{
 closure::Closure,
 JsCast
};
// use wasm_bindgen_futures::JsFuture;
use drop_ok::DropOk;
use drop_some::DropSome;
use web_sys::Window;

pub fn get_window() -> Result<Window, E>
{
 web_sys::window().ok_or(E::CouldNotRetrieveWindow)
}

#[derive(Debug)]
pub enum ContinueOrExit
{
 Continue,
 Exit
}

/// This will be run `f` only once. And the return the future object of `f`
pub async fn request_animation_frame_future<F, OK, ERROR>(mut f: F) -> Result<OK, ERROR>
where
 F: FnMut() -> Result<OK, ERROR> + 'static,
 OK: Copy + 'static,
 ERROR: Copy + 'static
{
 //let js_promise = js_sys::Promise::new(|resolve, reject| {} as dyn FnMut(js_sys::Function, js_sys::Function));
 // let promise = js_sys::Promise::resolve(&wasm_bindgen::JsValue::undefined());
 // let future = wasm_bindgen_futures::JsFuture::from(promise);

 // let c = Rc::new(RefCell::new(None));
 // let c_clone = c.clone();

 let r = Rc::new(RefCell::new(None));
 let r_clone = r.clone();

 let closure = Closure::wrap(Box::new(move || {
  *r_clone.borrow_mut() = Some(f());

 }) as Box<dyn FnMut()>);

 //*c.borrow_mut() = Some(closure);

 // request_animation_frame(c.borrow().as_ref().unwrap()).unwrap();
 request_animation_frame(&closure).unwrap();

 let x = r.borrow_mut().unwrap();
 x
}

/// Note: This will be invoke `f` repeatedly on a rendering frame.
/// ⚠ And the invocation loop be detach from a caller.
pub fn request_animation_frame_loop_detach<F>(mut f: F) -> Result<(), E>
where
 F: FnMut() -> Result<ContinueOrExit, E> + 'static
{
 let c = Rc::new(RefCell::new(None));
 let c_clone = c.clone();

 let closure = Closure::wrap(Box::new(move || {
  let continue_or_exit = f().unwrap();
  match continue_or_exit
  {
   ContinueOrExit::Continue => request_animation_frame(c_clone.borrow().as_ref().unwrap()).drop_ok().unwrap(),
   ContinueOrExit::Exit => c_clone.borrow_mut().take().drop_some().unwrap()
  };
 }) as Box<dyn FnMut()>);

 *c.borrow_mut() = Some(closure);

 request_animation_frame(c.borrow().as_ref().unwrap())?;

 Ok(())
}

/// - return: i32 => Request ID; it can use to the `cancel_animation_frame`
pub fn request_animation_frame(callback: &Closure<dyn FnMut()>) -> Result<i32, E>
{
 get_window()?
  .request_animation_frame(callback.as_ref().unchecked_ref())
  .map_err(|e| E::RequestAnimationFrameWasFailed(e))
}

pub fn cancel_animation_frame(id: i32) -> Result<(), E>
{
 get_window()?
  .cancel_animation_frame(id)
  .map_err(|e| E::CancelAnimationFrameWasFailed(e))
}

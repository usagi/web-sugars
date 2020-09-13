use crate::error::WebSysSugarsError as E;
use web_sys::{
 Document,
 Element,
 HtmlCollection,
 HtmlElement,
 XPathResult
};

pub fn get_document() -> Result<Document, E>
{
 crate::window::get_window()?.document().ok_or(E::CouldNotRetrieveDocument)
}

pub fn get_body() -> Result<HtmlElement, E>
{
 get_document()?.body().ok_or(E::CouldNotRetrieveDocumentBody)
}

pub fn get_element_by_id(id: &str) -> Result<Element, E>
{
 get_document()?.get_element_by_id(id).ok_or(E::ElementIdIsNotFound(id.into()))
}

pub fn generate_empty_html_collection() -> Result<HtmlCollection, E>
{
 Ok(get_document()?.create_document_fragment().children())
}

pub fn get_elements_by_class_name(class_names: &str) -> Result<HtmlCollection, E>
{
 Ok(get_document()?.get_elements_by_class_name(class_names))
}

pub fn get_elements_by_tag_name(tag_name: &str) -> Result<HtmlCollection, E>
{
 Ok(get_document()?.get_elements_by_tag_name(tag_name))
}

pub fn evaluate(expression: &str) -> Result<XPathResult, E>
{
 let d = get_document()?;
 d.evaluate(expression, &d).map_err(|e| E::EvaluateWasFailed(e))
}

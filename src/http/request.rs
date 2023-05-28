
pub use  super::method::Methods;

pub struct Request {
    path : String,
    query : Option<String>,
    method : Methods
}

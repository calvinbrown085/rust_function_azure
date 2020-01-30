use azure_functions::{
    bindings::{HttpRequest, HttpResponse},
    func,
};

#[func]
#[binding(name = "req", auth_level = "anonymous")]
pub fn hello(req: HttpRequest) -> HttpResponse {
    "Hello from Rust!".into()
}
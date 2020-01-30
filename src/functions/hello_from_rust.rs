use azure_functions::{
    bindings::{HttpRequest, HttpResponse},
    func,
};

#[func]
pub fn hello_from_rust(req: HttpRequest) -> HttpResponse {
    "Hello from Rust!".into()
}

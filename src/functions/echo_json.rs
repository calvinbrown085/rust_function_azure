use azure_functions::{
    bindings::{HttpRequest, HttpResponse},
    func,
};
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Error};
use azure_functions::http::Status;


#[derive(Deserialize)]
pub struct Request {
    pub value: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

#[func]
#[binding(name = "req", auth_level = "anonymous", route = "test-json-route")]
pub fn echo_json(req: HttpRequest) -> HttpResponse {
    let json_body: Result<Request, Error> = req.body().as_json::<Request>();

    match json_body {
        Ok(request) => {
            let resp = Response {
                message: format!("Echo, {}!", request.value),
            };
            to_value(resp).unwrap().into()
        },
        Err(e) => {
            HttpResponse::build()
                .status(Status::BadRequest)
                .body(format!("Invalid Json Request {}", e))
                .finish()
        },
    }
}
//
//#[cfg(test)]
//mod test {
//    use azure_functions::bindings::HttpRequest;
//
//    #[test]
//    fn oneequalsone() {
//        HttpRequest {
//
//        }
//        assert_eq!(1, 1);
//    }
//
//}
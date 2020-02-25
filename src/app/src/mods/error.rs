use actix_web::dev::HttpResponseBuilder;
use actix_web::HttpResponse;
use serde_json::json;

pub trait HttpResponseBuilderExt {
    fn error_response(&mut self, message: impl Into<String>, description: impl Into<String>) -> HttpResponse;
}

impl HttpResponseBuilderExt for HttpResponseBuilder {
    fn error_response(&mut self, message: impl Into<String>, description: impl Into<String>) -> HttpResponse {
        let body = json!({
            "message": message.into(),
            "description": description.into()
        });
        self.body(serde_json::to_string(&body).unwrap())
    }
}

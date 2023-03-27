use actix_web::{web, Responder, Scope};

use crate::{host::NativeHost, kernel::DummyKernel, kernel::Kernel};

async fn post_operation() -> impl Responder {
    println!("Operation has been submitted to the sequencer");
    let mut host = NativeHost {};
    DummyKernel::entry(&mut host);
    "Operation submitted"
}

/// Exposes all the endpoint of the application
pub fn service<K: Kernel>() -> Scope {
    web::scope("").route("/operations", web::post().to(post_operation))
}

#[cfg(test)]
mod tests {
    use crate::app;
    use actix_web::{
        body::MessageBody,
        http::{Method, StatusCode},
        test,
    };

    #[actix_web::test]
    async fn test_post_operation_content() {
        let app = test::init_service(app()).await;
        let req = test::TestRequest::with_uri("/operations")
            .method(Method::POST)
            .to_request();

        let resp = test::call_service(&app, req).await;

        let body = resp.into_body().try_into_bytes().unwrap().to_vec();
        let str = String::from_utf8(body).unwrap();

        assert_eq!(str, "Operation submitted")
    }

    #[actix_web::test]
    async fn test_post_operation_status() {
        let app = test::init_service(app()).await;
        let req = test::TestRequest::with_uri("/operations")
            .method(Method::POST)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}

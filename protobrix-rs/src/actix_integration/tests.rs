#[cfg(test)]
mod tests {
    use crate::builders::*;
    use crate::proto::*;
    use actix_web::{App, HttpResponse, test, web};
    use prost::Message;

    async fn test_handler() -> MainElement {
        StaticPageBuilder::new()
            .title("Test")
            .add_paragraph(ParagraphBuilder::new().add_text("Hello").build())
            .build()
    }

    async fn test_advanced_table_handler(request: AdvancedTableRequest) -> HttpResponse {
        HttpResponse::Ok().json(request)
    }

    #[actix_web::test]
    async fn test_responder_json() {
        let app = test::init_service(App::new().route("/test", web::get().to(test_handler))).await;

        let req = test::TestRequest::get()
            .uri("/test")
            .insert_header(("Accept", "application/json"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let content_type = resp.headers().get("content-type").unwrap();
        assert!(content_type.to_str().unwrap().contains("application/json"));

        let body = test::read_body(resp).await;
        let json_result: Result<MainElement, _> = serde_json::from_slice(&body);
        assert!(json_result.is_ok());
    }

    #[actix_web::test]
    async fn test_responder_protobuf() {
        let app = test::init_service(App::new().route("/test", web::get().to(test_handler))).await;

        let req = test::TestRequest::get()
            .uri("/test")
            .insert_header(("Accept", "application/x-protobuf"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let content_type = resp.headers().get("content-type").unwrap();
        assert!(
            content_type
                .to_str()
                .unwrap()
                .contains("application/x-protobuf")
        );

        let body = test::read_body(resp).await;
        let proto_result = MainElement::decode(body.as_ref());
        assert!(proto_result.is_ok());
    }

    #[actix_web::test]
    async fn test_responder_default_json() {
        let app = test::init_service(App::new().route("/test", web::get().to(test_handler))).await;

        let req = test::TestRequest::get().uri("/test").to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let content_type = resp.headers().get("content-type").unwrap();
        assert!(content_type.to_str().unwrap().contains("application/json"));
    }

    #[actix_web::test]
    async fn test_extractor_json() {
        let app = test::init_service(
            App::new().route("/test", web::post().to(test_advanced_table_handler)),
        )
        .await;

        let request_data = AdvancedTableRequest {
            columns: vec![],
            search: "test".to_string(),
            offset: 0,
            limit: 10,
        };

        let json_body = serde_json::to_string(&request_data).unwrap();

        let req = test::TestRequest::post()
            .uri("/test")
            .insert_header(("Content-Type", "application/json"))
            .set_payload(json_body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_extractor_protobuf() {
        let app = test::init_service(
            App::new().route("/test", web::post().to(test_advanced_table_handler)),
        )
        .await;

        let request_data = AdvancedTableRequest {
            columns: vec![],
            search: "test".to_string(),
            offset: 0,
            limit: 10,
        };

        let mut proto_body = Vec::new();
        request_data.encode(&mut proto_body).unwrap();

        let req = test::TestRequest::post()
            .uri("/test")
            .insert_header(("Content-Type", "application/x-protobuf"))
            .set_payload(proto_body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_extractor_missing_content_type() {
        let app = test::init_service(
            App::new().route("/test", web::post().to(test_advanced_table_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/test")
            .set_payload("{}")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 406); // Not Acceptable
    }

    #[actix_web::test]
    async fn test_extractor_unsupported_content_type() {
        let app = test::init_service(
            App::new().route("/test", web::post().to(test_advanced_table_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/test")
            .insert_header(("Content-Type", "text/plain"))
            .set_payload("test")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 406); // Not Acceptable
    }

    #[actix_web::test]
    async fn test_extractor_invalid_json() {
        let app = test::init_service(
            App::new().route("/test", web::post().to(test_advanced_table_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/test")
            .insert_header(("Content-Type", "application/json"))
            .set_payload("invalid json")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 400); // Bad Request
    }
}

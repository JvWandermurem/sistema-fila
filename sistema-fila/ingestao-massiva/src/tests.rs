use actix_web::{test, App};
use crate::models::PayloadData;
use crate::routes::health_check;

#[actix_web::test]
async fn test_health_check_route() {
    let app = test::init_service(App::new().service(health_check)).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_payload_deserialization() {
    let json_data = r#"
        {
            "user_id": "999",
            "action": "checkout",
            "timestamp": 1700000000
        }
    "#;

    let payload: Result<PayloadData, _> = serde_json::from_str(json_data);
    
    assert!(payload.is_ok());
    
    let parsed_data = payload.unwrap();
    assert_eq!(parsed_data.user_id, "999");
    assert_eq!(parsed_data.action, "checkout");
    assert_eq!(parsed_data.timestamp, 1700000000);
}
use axum::{Extension, extract::Path, Json };
use chrono::NaiveDateTime;
use entity::{order, user, review_slot};
use hyper::{Client, StatusCode, Request, Body, Response, Method, header};
use sea_orm::{DatabaseConnection, Set, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, QueryOrder};
use uuid::Uuid;
use hyper_tls::HttpsConnector;
use crate::{utils::{api_error::APIError, self}, models::order_model::{OrderModel, OrderApiReq, OrderApiRes, OrderApiSignature}};

pub async fn post_order(
    Extension(conn): Extension<DatabaseConnection>,
    Extension(identity): Extension<user::Model>, 
    Path(slot_uuid): Path<Uuid>
) -> Result<Json<OrderModel>,APIError>{

    let slot = review_slot::Entity::find()
    .filter(review_slot::Column::Uuid.eq(slot_uuid))
    .one(&conn).await
    .map_err(|err| APIError { error_code: Some(1), message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?
    .ok_or(APIError { error_code: None, message: "Resource Not Found".to_owned(), status_code: StatusCode::NOT_FOUND})?;

        
    let last_order  = order::Entity::find()
    .order_by_desc(order::Column::Id)
    .one(&conn).await
    .map_err(|err| APIError { error_code: Some(2), message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})? ;

    let receipt_no = match last_order {
        None => 0,
        _ => last_order.unwrap().id + 1 
    };



    let order_data: String = OrderApiReq {
        amount: 100000,
        currency: "INR".to_owned(),
        receipt: format!(r#"PF-{}"#,receipt_no)
    }.into();

    let https = HttpsConnector::new();
    let client: Client<_> = Client::builder().build::<_, hyper::Body>(https);


    let api_key = match *utils::constants::PRODUCTION {
        true => utils::constants::RAZORPAY_LIVE_KEY.to_string(),
        false => utils::constants::RAZORPAY_TEST_KEY.to_string()
    };

    let request = Request::builder()
    .header(header::CONTENT_TYPE, "application/json")
    .header(header::AUTHORIZATION, format!(r#"Basic {}"#,api_key))
    .header(header::HOST, "api.razorpay.com")
    .uri("https://api.razorpay.com/v1/orders")
    .method(Method::POST)
    .body(Body::from(order_data))
    .map_err(|err| APIError { error_code: Some(3), message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})? ;

    let res = client.request(request).await
    .map_err(|err| APIError { error_code: Some(4), message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;
    

    let order_api_res = order_res_to_order_obj(res).await
    .map_err(|err| APIError { error_code: Some(5), message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;

    let order_entity = order::ActiveModel {
        order_id: Set(order_api_res.id),
        user_id: Set(identity.id),
        currency: Set(order_api_res.currency),
        amount: Set(order_api_res.amount),
        amount_paid: Set(order_api_res.amount_paid),
        amount_due: Set(order_api_res.amount_due),
        receipt:Set(order_api_res.receipt),
        offer_id: Set(order_api_res.offer_id),
        status: Set(order_api_res.status),
        attempts: Set(order_api_res.attempts),
        updated_at: Set(NaiveDateTime::from_timestamp_opt(order_api_res.created_at, 0).unwrap()),
        created_at: Set(NaiveDateTime::from_timestamp_opt(order_api_res.created_at, 0).unwrap()),
        slot_id: Set(slot.id),
        ..Default::default()
    }.insert(&conn).await
    .map_err(|err| APIError { error_code: Some(6), message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;

    let response = OrderModel {
        id: order_entity.id,
        order_id: order_entity.order_id,
        user_id: identity.id,
        currency: order_entity.currency,
        amount: order_entity.amount,
        amount_paid: order_entity.amount_paid,
        amount_due: order_entity.amount_due,
        receipt:order_entity.receipt,
        offer_id: order_entity.offer_id,
        status: order_entity.status,
        attempts: order_entity.attempts,
        updated_at: order_entity.created_at,
        created_at: order_entity.created_at,
        slot_id: slot.id,
    };

    Ok(Json(response))

} 
async fn order_res_to_order_obj(
    res: Response<Body>
) 
-> Result<OrderApiRes, Box<dyn std::error::Error + Send + Sync>> {

    let response_bytes = hyper::body::to_bytes(res.into_body()).await?;
    let response_str = std::str::from_utf8(&response_bytes)?;
    let parsed_json: OrderApiRes = serde_json::from_str(response_str)?;
    Ok(parsed_json)
}

pub async fn post_order_signature(
    Extension(conn): Extension<DatabaseConnection>, 
    // Extension(identity): Extension<Model>,
    Json(order_signature): Json<OrderApiSignature>
) -> Result<(),APIError>{

    let verify = razorpay::utils::verify_webhook_signature(&format!(r#"{}|{}"#,order_signature.order_id,order_signature.payment_id),&order_signature.signature,&*utils::constants::RAZORPAY_TEST_SECRET_KEY);

    match  verify {
        Ok(()) => { println!("Signature Match") },
        Err(_) => return Err(APIError { error_code: None, message: "Incorrect Signature".to_string(), status_code: StatusCode::FORBIDDEN})
        
    }

    let order_data = order::Entity::find().filter(order::Column::OrderId.eq(order_signature.order_id)).one(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?
    .ok_or(APIError { error_code: None, message: "Resource Not Found".to_owned(), status_code: StatusCode::NOT_FOUND})?;

    let mut order: order::ActiveModel = order_data.clone().into();
   
    
   order.status = Set("paid".to_string());
   order.amount_paid = Set(order_data.amount);
   order.amount_due = Set(0);
   order.update(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;

    Ok(())
}
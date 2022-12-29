use crate::errors::ApiError;
use candid::Principal;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use ic_kit::ic;
use uuid::Uuid;

use crate::domain::sms::types::Sms;

const COURIER_SEND_URL: &str = "https://api.courier.com/send";

pub async fn send_courier_sms(api_key: &str, sms: &Sms) -> Result<(), ApiError> {
    let (bytes,): (Vec<u8>,) = ic::call(Principal::management_canister(), "raw_rand", ())
        .await
        .map_err(|(_, _)| ApiError::InternalError)?;

    let idempotency_key = Uuid::from_slice(&(bytes)[..16])
        .map_err(|_| ApiError::InternalError)?
        .to_string();

    ic::print("Despues de armar la idempotency key");

    let host = String::from(COURIER_SEND_URL);

    let request_headers: Vec<HttpHeader> = vec![
        HttpHeader {
            name: "Authorization".to_owned(),
            value: format!("Bearer {}", api_key.clone()),
        },
        HttpHeader {
            name: "Idempotency-Key".to_owned(),
            value: idempotency_key,
        },
    ];

    ic::print("Antes de armar el argument");

    let request = CanisterHttpRequestArgument {
        url: host,
        method: HttpMethod::POST,
        body: Some(sms.to_courier_format().into_bytes()),
        max_response_bytes: None,
        transform: Some(TransformContext::new(transform_send_sms, vec![])),
        headers: request_headers,
    };

    ic::print(format!("{:?}", request));

    match http_request(request).await {
        Ok((response,)) => {
            ic::print(format!("{:?}", response));
        }
        Err((r, m)) => {
            ic::print(format!("{:?} {:?}", r, m));
        }
    };

    Ok(())
}

#[ic_cdk_macros::query]
pub fn transform_send_sms(raw: TransformArgs) -> HttpResponse {
    let mut sanitized = raw.response.clone();
    sanitized.headers = vec![];
    sanitized
}
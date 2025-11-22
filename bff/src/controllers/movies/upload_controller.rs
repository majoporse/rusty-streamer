use std::time;

use crate::models::movies::{UploadSasRequest, UploadSasResponse};

use actix_web::{post, web};
use azure_core::time::OffsetDateTime;
use azure_storage::prelude::{BlobSasPermissions, SasToken};
use azure_storage::ConnectionString;
use azure_storage_blobs::prelude::ClientBuilder;
use utoipa::OpenApi;

static TAG: &str = "Uploads";

#[utoipa::path(
    request_body = UploadSasRequest,
    responses(
        (status = 200, description = "SAS token for upload", body = UploadSasResponse),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/uploads/request-sas")]
pub async fn request_upload_sas(
    req: web::Json<UploadSasRequest>,
) -> Result<web::Json<UploadSasResponse>, actix_web::Error> {
    let account = std::env::var("AZURE_ACCOUNT_NAME").expect("Missing AZURE_ACCOUNT_NAME");
    let access_key = std::env::var("AZURE_ACCOUNT_KEY").expect("Missing AZURE_ACCOUNT_KEY");
    let container = std::env::var("AZURE_CONTAINER_NAME").expect("Missing AZURE_CONTAINER_NAME");
    let con_str = std::env::var("AZURE_CONN_STRING").expect("Missing AZURE_CONN_STRING");

    let conn = ConnectionString::new(&con_str).expect("Invalid connection string");
    let storage_credentials = conn
        .storage_credentials()
        .expect("Missing storage credentials");

    let builder = ClientBuilder::new(&account, storage_credentials).cloud_location(
        azure_storage::CloudLocation::Custom {
            account: "devstoreaccount1".to_string(),
            uri: "http://127.0.0.1:10000/devstoreaccount1".to_owned(),
        },
    );

    let client = builder.clone().blob_service_client();
    let blob_name = format!("uploads/{}", req.filename);

    let blob_client = client.container_client(&container).blob_client(&blob_name);

    let permissions = BlobSasPermissions {
        read: true,
        write: true,
        create: true,
        ..Default::default()
    };

    let expiry = OffsetDateTime::now_utc() + time::Duration::from_secs(60 * 60);

    let token = blob_client
        .shared_access_signature(permissions, expiry)
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to generate SAS token: {}",
                e
            ))
        })?;
    let token_str = token.token().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Failed to get token: {}", e))
    })?;

    let url = blob_client.url().unwrap().to_string();

    let res = UploadSasResponse {
        upload_url: format!("{}?{}", url, token_str),
        blob_url: url,
        expires_at: Some(expiry.to_string()),
    };

    Ok(web::Json(res))
}

#[derive(OpenApi)]
#[openapi(paths(request_upload_sas))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(request_upload_sas);
}

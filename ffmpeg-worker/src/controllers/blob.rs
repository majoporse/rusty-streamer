use std::path::PathBuf;

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{error::ErrorInternalServerError, put, Error, HttpResponse};
use anyhow::{Context, Result};
use azure_storage::ConnectionString;
use azure_storage_blobs::prelude::{BlobServiceClient, ClientBuilder};
use futures::stream::StreamExt;
use log::{debug, info};
use tokio::io::AsyncReadExt;
use walkdir::WalkDir;

use crate::logic::ffmpeg;

#[derive(Debug, MultipartForm, utoipa::ToSchema)]
#[allow(unused)]
pub struct UploadForm {
    #[multipart(limit = "5 GiB")]
    #[schema(value_type = String, format = Binary, content_media_type = "application/octet-stream")]
    file: TempFile,

    #[schema(value_type = String, content_media_type = "text/plain")]
    json: Text<String>,
}

#[utoipa::path(
    request_body(content_type = "multipart/form-data", content = UploadForm),
    responses(
        (status = 200, description = "Upload file to Azure Blob Storage")
    )
)]
#[put("/file")]
async fn upload(MultipartForm(form): MultipartForm<UploadForm>) -> Result<HttpResponse, Error> {
    handle_upload(form).await.map_err(|e| ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}

async fn handle_upload(form: UploadForm) -> Result<()> {
    let file_name = form.json.clone();

    // --- Environment setup ---
    let account = std::env::var("AZURE_ACCOUNT_NAME").context("Missing AZURE_ACCOUNT_NAME")?;
    let access_key = std::env::var("AZURE_ACCOUNT_KEY").context("Missing AZURE_ACCOUNT_KEY")?;
    let container = std::env::var("AZURE_CONTAINER_NAME").context("Missing AZURE_CONTAINER_NAME")?;
    let con_str = std::env::var("AZURE_CONN_STRING").context("Missing AZURE_CONN_STRING")?;

    let blob_name = format!("uploads/{}", file_name);

    let conn = ConnectionString::new(&con_str).context("Invalid connection string")?;
    let storage_credentials = conn.storage_credentials().context("Missing storage credentials")?;

    let builder = ClientBuilder::new(&account, storage_credentials.clone()).cloud_location(
        azure_storage::CloudLocation::Custom {
            account: "devstoreaccount1".to_string(),
            uri: "http://127.0.0.1:10000/devstoreaccount1".to_owned(),
        },
    );
    let client = builder.clone().blob_service_client();

    client.container_client(&container).create().await.ok();
    info!("Container created or already exists");

    debug!("account: {}, container: {}", account, container);
    debug!("blob: {}", &blob_name);
    debug!("key: {}", access_key);

    let path = form.file.file.path().to_path_buf();

    // --- Process the file with ffmpeg ---
    ffmpeg::convert_to_hls(&path, format!("output_hls/{}", file_name)).await?;

    // --- Upload HLS output folder ---
    let upload_path = PathBuf::from(format!("output_hls/{}", file_name));
    upload_folder(&client, &container, &upload_path).await?;

    info!("Uploaded folder: {:?}", upload_path);

    Ok(())
}

async fn upload_file(
    client: &BlobServiceClient,
    container: &str,
    file_path: &PathBuf,
) -> Result<()> {
    let file_name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .context("Invalid file name")?;

    let folder_name = file_path
        .parent()
        .and_then(|p| p.to_str())
        .context("Invalid folder name")?;

    let blob_client = client
        .container_client(container)
        .blob_client(format!("{}/{}", folder_name, file_name));

    let mut file = tokio::fs::File::open(file_path)
        .await
        .context(format!("Failed to open file {:?}", file_path))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .await
        .context("Failed to read file contents")?;

    blob_client
        .put_block_blob(buffer)
        .content_type("application/octet-stream")
        .await
        .context("Failed to upload blob")?;

    info!("Uploaded file: {}", file_name);
    Ok(())
}

async fn upload_folder(client: &BlobServiceClient, container: &str, folder: &PathBuf) -> Result<()> {
    for entry in WalkDir::new(folder) {
        let item = entry.context("Failed to walk directory")?;
        if item.file_type().is_dir() {
            continue;
        }
        upload_file(client, container, &item.into_path())
            .await
            .context("Failed to upload file from folder")?;
    }
    Ok(())
}

#[allow(unused)]
async fn fetch_blob(client: &BlobServiceClient, container: &str, blob: &str) -> Result<Vec<u8>> {
    let blob_client = client.container_client(container).blob_client(blob);
    let mut result: Vec<u8> = vec![];
    let mut stream = blob_client.get().into_stream();

    while let Some(value) = stream.next().await {
        let mut body = value.context("Failed to fetch blob")?.data;

        while let Some(value) = body.next().await {
            let value = value.context("Failed to read blob body chunk")?;
            result.extend(&value);
        }
    }
    Ok(result)
}

async fn list_blobs(client: &BlobServiceClient, container: &str) -> Result<()> {
    let container_client = client.container_client(container);
    let mut stream = container_client.list_blobs().into_stream();

    while let Some(value) = stream.next().await {
        let blobs = value.context("Failed to list blobs")?;
        for blob in blobs.blobs.blobs() {
            info!("Blob name: {}", blob.name);
        }
    }
    Ok(())
}

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(upload);
}

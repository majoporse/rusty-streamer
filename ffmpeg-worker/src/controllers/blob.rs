use std::path::PathBuf;

use azure_storage::ConnectionString;
use azure_storage_blobs::prelude::{BlobServiceClient, ClientBuilder};
use futures::stream::StreamExt;

use actix_web::{error::ErrorInternalServerError, put, Error, HttpResponse};

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
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
    let file_name = form.json.clone();

    // First we retrieve the account name and access key from environment variables.
    let account = std::env::var("AZURE_ACCOUNT_NAME").expect("missing AZURE_ACCOUNT_NAME");
    let access_key = std::env::var("AZURE_ACCOUNT_KEY").expect("missing AZURE_ACCOUNT_KEY");
    let container = std::env::var("AZURE_CONTAINER_NAME").expect("missing AZURE_CONTAINER_NAME");
    let con_str = std::env::var("AZURE_CONN_STRING").unwrap();
    let blob_name = "uploads/".to_string() + &file_name;

    let conn = ConnectionString::new(&*con_str).expect("invalid connection string");
    let storage_credentials = conn.storage_credentials().expect("msg");

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

    // let (file, path) = form.file.file.keep().map_err(|e| ErrorInternalServerError(e))?;
    let path = form.file.file.path().to_path_buf();

    ffmpeg::convert_to_hls(&path, "output_hls/".to_string() + &file_name.clone()).await?;

    // upload_file(&client, &container, &path).await?;

    // tokio::fs::remove_file(path).await.map_err(|e| ErrorInternalServerError(e))?;
    let upload_path = PathBuf::from("output_hls/".to_string() + &file_name.clone());

    upload_folder(&client, &container, &upload_path).await?;
    info!("Uploaded folder: {:?}", upload_path);

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}

async fn upload_file(
    client: &BlobServiceClient,
    container: &str,
    file_path: &PathBuf,
) -> Result<(), Error> {
    let file_name = file_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| ErrorInternalServerError("Invalid file name"))?;

    let folder_name = file_path
        .parent()
        .ok_or_else(|| ErrorInternalServerError("Invalid folder name"))?
        .to_str()
        .ok_or_else(|| ErrorInternalServerError("Invalid folder name"))?;

    let blob_client = client
        .container_client(container)
        .blob_client( folder_name.to_string() + "/" + file_name);

    let mut file = tokio::fs::File::open(file_path)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;

    blob_client
        .put_block_blob(buffer)
        .content_type("application/octet-stream")
        .await
        .map_err(|e| ErrorInternalServerError(e))?;

    info!("Uploaded file: {}", file_name);
    Ok(())
}

#[allow(unused)]
async fn fetch_blob(
    client: &BlobServiceClient,
    container: &str,
    blob: &str,
) -> Result<Vec<u8>, Error> {
    let blob_client = client.container_client(container).blob_client(blob);
    let mut result: Vec<u8> = vec![];

    // The stream is composed of individual calls to the get blob endpoint
    let mut stream = blob_client.get().into_stream();

    while let Some(value) = stream.next().await {
        let mut body = value.map_err(|e| ErrorInternalServerError(e))?.data;

        // For each response, we stream the body instead of collecting it all
        // into one large allocation.
        while let Some(value) = body.next().await {
            let value = value.map_err(|e| ErrorInternalServerError(e))?;
            result.extend(&value);
        }
    }
    Ok(result)
}

async fn upload_folder(
    client: &BlobServiceClient,
    container: &str,
    folder: &PathBuf,
) -> Result<(), Error> {
    for entry in WalkDir::new(folder) {
        info!("Uploading entry: {:?}", entry);
        let item = entry.map_err(|e| ErrorInternalServerError(e))?;
        if item.file_type().is_dir() {
            continue;
        }
        upload_file(client, container, &item.into_path()).await?;
    }
    Ok(())
}


async fn list_blobs(cliennt: &BlobServiceClient, cotainer:&str) -> Result<(), Error> {
    let container_client = cliennt.container_client(cotainer);
    let mut stream = container_client.list_blobs().into_stream();

    while let Some(value) = stream.next().await {
        let blobs = value.map_err(|e| ErrorInternalServerError(e))?;
        for blob in blobs.blobs.blobs() {
            info!("Blob name: {}", blob.name);
            // if (blob.properties.)
        }
    }
    Ok(())
}

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(upload);
}

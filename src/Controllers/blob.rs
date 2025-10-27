use actix_multipart::form::{json::Json as MPJson, tempfile::TempFile, MultipartForm};
use utoipa_actix_web::service_config::ServiceConfig;

// #[derive(Debug, MultipartForm)]
// struct UploadForm {
//     #[multipart(limit = "100MB")]
//     file: TempFile,
//     json: MPJson<Metadata>,
// }
//
// #[utoipa::path(
//     responses(
//         (status = 200, description = "Upload file to Azure Blob Storage")
//     )
// )]
// #[put("/upload_file")]
// async fn upload_file(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
//     let file_name = "azure_sdk_for_rust_stream_test.txt";
//
//     // First we retrieve the account name and access key from environment variables.
//     let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
//     let access_key = std::env::var("STORAGE_ACCESS_KEY").expect("missing STORAGE_ACCOUNT_KEY");
//     let container = std::env::var("STORAGE_CONTAINER").expect("missing STORAGE_CONTAINER");
//     let blob_name = std::env::var("STORAGE_BLOB_NAME").expect("missing STORAGE_BLOB_NAME");
//
//     let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
//     let blob_client =
//         ClientBuildear::new(account, storage_credentials).blob_client(&container, blob_name);
//
//     blob_client
//         .put_block_blob("hello world")
//         .content_type("text/plain")
//         .await?;
//
//     let mut result: Vec<u8> = vec![];
//
//     // The stream is composed of individual calls to the get blob endpoint
//     let mut stream = blob_client.get().into_stream();
//     while let Some(value) = stream.next().await {
//         let mut body = value?.data;
//         // For each response, we stream the body instead of collecting it all
//         // into one large allocation.
//         while let Some(value) = body.next().await {
//             let value = value?;
//             result.extend(&value);
//         }
//     }
//
//     println!("result: {:?}", result);
//
//     HttpResponse::Ok().body("File uploaded")
// }

pub fn scoped_config(cfg: &mut ServiceConfig) {}

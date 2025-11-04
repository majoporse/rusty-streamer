use log::debug;
use opentelemetry::{
    global,
    trace::{FutureExt as _, Span, TraceContextExt as _, Tracer},
    Context, KeyValue,
};
use std::path::Path;
use tokio::process::Command;

pub async fn convert_to_hls(input_path: &Path, output_dir: String) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(&output_dir)?;

    let input = input_path.to_str().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid input path")
    })?;
    let azure_user =
        std::env::var("AZURE_STORAGE_ACCOUNT").unwrap_or_else(|_| "devstoreaccount1".to_string());

    let mut com = Command::new("ffmpeg");
    com.args(&[
        "-y", // Overwrite output files
        // "-loglevel", "debug",

        "-vsync", "0",
        "-hwaccel", "cuda",
        "-hwaccel_output_format", "cuda",
        "-i", input,
        
        "-filter_complex", "\
            [0:v]hwdownload,format=nv12,split=4[v0][v1][v2][v3];\
            [v0]scale=1920:1080[v0out];\
            [v1]scale=1280:720[v1out];\
            [v2]scale=854:480[v2out];\
            [v3]scale=640:360[v3out]\
        ",
        // Video streams
        "-map", "[v0out]", "-map", "0:a?",
        "-map", "[v1out]", "-map", "0:a?",
        "-map", "[v2out]", "-map", "0:a?",
        "-map", "[v3out]", "-map", "0:a?",

        // Video encoding
        "-c:v", "h264_nvenc",
        "-preset:v", "fast", 
        "-rc:v", "vbr", 
        "-cq:v", "21",

        // Audio
        "-c:a", "aac", "-b:a", "192k",

        // HLS settings
        "-hls_time", "6",
        "-hls_playlist_type", "vod",
        "-hls_segment_filename", &format!("{}/segment_%v_%03d.ts", output_dir),
        "-master_pl_name", "master.m3u8", // optional, can be removed

        "-var_stream_map", "v:0,a:0 v:1,a:1 v:2,a:2 v:3,a:3",
        "-hls_base_url", &format!("http://{}/{}/{}/{}/", "localhost:10000", azure_user, "video", &output_dir),
        "-f", "hls",
        &(output_dir.clone() + "/" + "stream_%v.m3u8"),
    ]);

    debug!("Running FFmpeg command: {:?}", com);

    let tracer = global::tracer("ffmpeg_worker");
    let context = Context::current();
    let mut span = tracer.start("ffmpeg_conversion");

    span.set_attributes(vec![
        opentelemetry::KeyValue::new("input_path", input.to_string()),
        opentelemetry::KeyValue::new("output_dir", output_dir.clone()),
    ]);

    let status = com.status().with_context(context).await?;

    span.set_attribute(opentelemetry::KeyValue::new(
        "exit_status",
        format!("{:?}", status),
    ));

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "FFmpeg command failed",
        ));
    }

    Ok(())
}

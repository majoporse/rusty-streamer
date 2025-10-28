use std::path::Path;
use log::debug;
use tokio::process::Command;

pub async fn convert_to_hls(input_path: &Path, output_dir: String) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(&output_dir)?;

    let output_path = format!("{}/master.m3u8", output_dir);
    let input = input_path.to_str().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid input path")
    })?;

    // Filter complex: download frames to CPU, then split and scale
    let filter_complex = "\
        [0:v]hwdownload,format=nv12,split=4[v0][v1][v2][v3];\
        [v0]scale=1920:1080[v0out];\
        [v1]scale=1280:720[v1out];\
        [v2]scale=854:480[v2out];\
        [v3]scale=640:360[v3out]\
    ";

    let mut com = Command::new("ffmpeg");
    com.args(&[
        "-y",                      // Overwrite output files
        // "-loglevel", "debug",
        "-vsync", "0",
        "-hwaccel", "cuda",
        "-hwaccel_output_format", "cuda",
        "-i", input,
        "-filter_complex", filter_complex,

        // Video encoding per variant
        "-map", "[v0out]", "-map", "0:a", "-c:v:0", "h264_nvenc", "-preset:v:0", "fast", "-rc:v:0", "vbr", "-cq:v:0", "21", "-b:v:0", "5000k",
        "-map", "[v1out]", "-map", "0:a", "-c:v:1", "h264_nvenc", "-preset:v:1", "fast", "-rc:v:1", "vbr", "-cq:v:1", "21", "-b:v:1", "3000k",
        "-map", "[v2out]", "-map", "0:a", "-c:v:2", "h264_nvenc", "-preset:v:2", "fast", "-rc:v:2", "vbr", "-cq:v:2", "21", "-b:v:2", "1500k",
        "-map", "[v3out]", "-map", "0:a", "-c:v:3", "h264_nvenc", "-preset:v:3", "fast", "-rc:v:3", "vbr", "-cq:v:3", "21", "-b:v:3", "800k",

        // Audio
        "-c:a", "aac", "-b:a", "192k",

        // HLS settings
        "-hls_time", "6",
        "-hls_playlist_type", "vod",
        "-hls_segment_filename", &format!("{}/segment_%v_%03d.ts", output_dir),
        "-master_pl_name", "master.m3u8", // optional, can be removed

        "-var_stream_map", "v:0,a:0 v:1,a:1 v:2,a:2 v:3,a:3",
        "-f", "hls",
        &(output_path + "stream_%v.m3u8"),
    ]);

    debug!("Running FFmpeg command: {:?}", com);
    let status = com.status().await?;

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "FFmpeg command failed",
        ));
    }

    Ok(())
}

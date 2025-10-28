use std::path::Path;

use log::debug;
use tokio::process::Command;

pub async fn convert_to_hls(input_path: &Path, output_dir: String) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(&output_dir)?;

    let output_path = format!("{}/master.m3u8", output_dir);
    let input = input_path.to_str().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid input path")
    })?;

    let mut com = Command::new("ffmpeg");
    com
        .args(&[
            "-vsync", "0",
            "-hwaccel", "cuda",
            "-hwaccel_output_format", "cuda",
            "-i", input,
            "-c:v", "h264_nvenc",
            "-c:a", "copy",
            "-profile:v", "main",
            "-preset", "fast",
            "-crf", "23",
            "-sc_threshold", "0",
            "-g", "48",    
            "-keyint_min", "48",
            "-threads", "16", 
            "-hls_time", "4",
            "-hls_playlist_type", "vod",
            "-hls_segment_filename", &format!("{}/segment_%03d.ts", output_dir),
            "-master_pl_name", "master.m3u8",
            "-var_stream_map", "v:0,a:0",
            "-f", "hls",
            &output_path,
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
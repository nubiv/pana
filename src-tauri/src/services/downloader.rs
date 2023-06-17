use anyhow::anyhow;
use futures_util::TryStreamExt;
use tokio::io::AsyncWriteExt;

use crate::app_event;
use crate::utils::errors::DownloadError;
use crate::utils::events::*;

pub async fn download(
    window: &tauri::Window,
    bin_path: &std::path::Path,
    model_info: &crate::utils::models::ModelInfo,
) -> Result<(), DownloadError> {
    let model_filename = bin_path.join(&model_info.filename);

    let portion = match tokio::fs::File::open(&model_filename).await {
        Ok(file) => {
            let metadata = file.metadata().await.unwrap();

            metadata.len()
        }
        Err(e) => {
            println!("{}", e);

            0
        }
    };

    let client = reqwest::Client::new();
    let res = client
        .get(&model_info.download_url)
        .header("Range", format!("bytes={}-", &portion))
        .send()
        .await;

    let mut model = tokio::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(&model_filename)
        .await?;

    let length = client
        .get(&model_info.download_url)
        .send()
        .await
        .map_err(|e| {
            DownloadError::Custom(anyhow!(
                "failed to get content length. {}",
                e
            ))
        })?
        .content_length()
        .unwrap() as f64;

    match res {
        Ok(res) => {
            println!("download started.");

            let mut stream = res.bytes_stream();

            let mut progress = portion as f64;

            while let Some(chunk) = stream.try_next().await? {
                if let Err(e) = model.write_all(&chunk).await {
                    println!("error out {}", e);
                };

                progress += chunk.len() as f64;

                let percentage = progress / length * 100.0;

                app_event!(
                    window,
                    Download,
                    DownloadPayload {
                        progress: percentage
                    }
                );
            }

            println!("download completed.");

            Ok(())
        }
        Err(e) => Err(DownloadError::Custom(anyhow::Error::new(e))),
    }
}

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

    let chunk_in_place = match tokio::fs::File::open(&model_filename).await {
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
        .header("Range", format!("bytes={}-", &chunk_in_place))
        .send()
        .await;

    let mut model = tokio::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(&model_filename)
        .await?;

    match res {
        Ok(res) => {
            let mut stream = res.bytes_stream();

            let mut size = chunk_in_place;

            while let Some(chunk) = stream.try_next().await? {
                if let Err(e) = model.write_all(&chunk).await {
                    println!("error out {}", e);
                };

                size += chunk.len() as u64;

                app_event!(window, Download, DownloadPayload { size });
            }

            // while stream.try_next().await.is_ok() {
            //     while let Some(chunk) = chunk_option.take() {
            //         if let Err(e) = model.write_all(&chunk).await {
            //             println!("error out {}", e);

            //             app_event!(
            //                     window,
            //                     Error,
            //                     ErrorPayload {
            //                         message: String::from(
            //                             "Something went wrong while downloading, please try again."
            //                         )
            //                     }
            //                 );

            //             return Ok(());
            //         };

            //         size += chunk.len() as u64;

            //         app_event!(window, Download, DownloadPayload { size });
            //     }
            // } else {

            // };

            println!("download completed.");
            app_event!(
                window,
                Noticification,
                NoticificationPayload {
                    message: String::from("Download completed.")
                }
            );

            Ok(())
        }
        Err(e) => Err(DownloadError::Custom(anyhow::Error::new(e))),
    }
}

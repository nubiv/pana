use futures_util::TryStreamExt;
use tokio::io::AsyncWriteExt;

pub async fn download(app_handle: &tauri::AppHandle, window: &tauri::Window) {
    let app_data_dir = app_handle.path_resolver().app_data_dir().unwrap();
    let llm_path = app_data_dir.join("llm");
    let model_path = llm_path.join("wizardLM-7B.ggml.q4_0.bin");

    let portion = match tokio::fs::File::open(&model_path).await {
        Ok(file) => {
            let metadata = file.metadata().await.unwrap();
            println!("metadata: {:?}", metadata.len());

            metadata.len()
        }
        Err(e) => {
            println!("{}", e);

            0
        }
    };

    // replace the following line with AppEvent::<Download>
    window.emit("system", portion).unwrap();

    let client = reqwest::Client::new();
    let res = client
        .get("https://huggingface.co/TheBloke/wizardLM-7B-GGML/resolve/previous_llama_ggmlv2/wizardLM-7B.ggml.q4_0.bin")
        .header("Range", format!("bytes={}-", &portion))
        .send()
        .await;

    let mut model = tokio::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(&model_path)
        .await
        .unwrap();

    match res {
        Ok(res) => {
            let length = res.content_length().unwrap() as f64;
            println!("content-length {}", length);

            let mut stream = res.bytes_stream();

            let mut progress = portion as f64;

            while let Some(chunk) = stream.try_next().await.unwrap() {
                if let Err(e) = model.write_all(&chunk).await {
                    println!("error out {}", e);
                };

                progress += chunk.len() as f64;
                let percentage = format!("{:.2}", progress / length);
                println!("progress>>> {}", percentage);
            }

            println!("download completed.");
        }
        Err(e) => println!("dowanload err {}", e),
    }
}

use std::{error::Error, fs};

pub fn is_llm_dir_existed(app_handle: &tauri::AppHandle) -> bool {
    let app_data_dir = app_handle.path_resolver().app_data_dir().unwrap();
    let llm_dir = app_data_dir.join("llm");

    match fs::read_dir(llm_dir) {
        Ok(_) => true,
        Err(e) => {
            println!("is llm dir existed error: {}", e);
            false
        }
    }
}

pub fn create_llm_dir(app: &tauri::AppHandle) -> Result<(), Box<dyn Error>> {
    let app_data_dir = app.path_resolver().app_data_dir().unwrap();
    let llm_dir = app_data_dir.join("llm");

    match fs::create_dir(llm_dir) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn find_local_models(app: &tauri::AppHandle) -> Result<Vec<String>, ()> {
    let app_data_dir = app.path_resolver().app_data_dir().unwrap();
    let llm_dir = app_data_dir.join("llm");

    let mut models = vec![];
    let files = match fs::read_dir(llm_dir) {
        Ok(files) => files,
        Err(_) => return Err(()),
    };
    // Iterate over the files and return the first `.bin` file found
    for file in files {
        let path = match file {
            Ok(file) => file.path(),
            Err(_) => continue,
        };
        let extension = match path.extension() {
            Some(extension) => extension.to_string_lossy(),
            None => continue,
        };
        if extension == "bin" {
            models.push(path.to_str().unwrap().to_string());
        }
    }

    Ok(models)
}

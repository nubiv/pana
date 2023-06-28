use std::collections::VecDeque;

use crate::utils::errors::AppError;

pub fn init_db(
    app_handle: &tauri::AppHandle,
) -> Result<sled::Db, AppError> {
    let db_path = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join("db");

    let db =
        sled::Config::default().path(db_path).open()?;

    Ok(db)
}

pub fn setup_tree(
    db: &sled::Db,
) -> Result<sled::Tree, AppError> {
    // TODO: add a session id to distinguish between sessions
    let local_time = chrono::Local::now();
    let formatted_time =
        local_time.format("%Y-%m-%d").to_string();
    let tree = db.open_tree(formatted_time)?;

    Ok(tree)
}

pub fn get_latest_adjacency_pairs(
    tree: &sled::Tree,
) -> Result<VecDeque<(String, String)>, AppError> {
    // for (idx, kv) in tree.iter().enumerate() {
    //     let kv = kv.unwrap();
    //     println!("{}: ", idx);
    //     print_kv(&kv.0, &kv.1);
    // }
    let mut pairs = VecDeque::new();
    tree.iter()
        .rev()
        .take(4)
        .map(|kv| {
            let kv = kv.unwrap();
            let k_str = match std::str::from_utf8(&kv.0)
                .unwrap()
                .ends_with('0')
            {
                true => "### Human".to_owned(),
                false => "### Pana".to_owned(),
            };
            let v_str = std::str::from_utf8(&kv.1)
                .unwrap()
                .to_owned();

            (k_str, v_str)
        })
        .for_each(|x| pairs.push_front(x));

    Ok(pairs)
}

pub fn insert_adjacency_pair(
    user_formatted_time: &str,
    user_message: &str,
    pana_formatted_time: &str,
    pana_message: &str,
    tree: &sled::Tree,
) -> Result<(), AppError> {
    let mut batch = sled::Batch::default();
    batch.insert::<&str, &str>(
        user_formatted_time,
        user_message,
    );
    batch.insert::<&str, &str>(
        pana_formatted_time,
        pana_message,
    );
    tree.apply_batch(batch)?;

    Ok(())
}

// pub fn print_all_messages(db: &sled::Db) {
//     for kv in db.iter() {
//         let kv = kv.unwrap();
//         let k_str = std::str::from_utf8(&kv.0).unwrap();
//         let v_str = std::str::from_utf8(&kv.1).unwrap();
//         let str = (k_str, v_str);
//         println!("{:?}", str);
//     }
// }

pub fn print_kv(k: &sled::IVec, v: &sled::IVec) {
    let k_str = std::str::from_utf8(k).unwrap();
    let v_str = std::str::from_utf8(v).unwrap();
    let str = (k_str, v_str);
    println!("{:?}", str);
}

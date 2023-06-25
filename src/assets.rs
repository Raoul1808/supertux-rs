use std::env::current_exe;
use std::path::{Path, PathBuf};

fn get_assets_path() -> PathBuf {
    let mut path = current_exe().unwrap();

    #[cfg(debug_assertions)]
    {
        path = path.parent().and_then(Path::parent).unwrap().to_path_buf();
    }

    path.parent().unwrap().join("res").to_path_buf()
}

pub async fn get_asset_bytes(asset_name: &str) -> Vec<u8> {

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let uri = web_sys::window().unwrap().location().href().unwrap();
            reqwest::get(format!("{}/res/{}", uri, asset_name))
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap()
                .to_vec()
        } else {
            let path = get_assets_path().join(asset_name);
            std::fs::read(path).unwrap()
        }
    }
}

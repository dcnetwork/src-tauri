// #![cfg_attr(
//   all(not(debug_assertions), target_os = "windows"),
//   windows_subsystem = "windows"
// )]

use node::stun::connect::*;
use node::config::init::*;
use serde::Serialize;
use hex::*;

#[derive(Serialize,Debug)]
pub struct UserInfo{
    address: String,
    pubkey: Vec<u8>
}


#[tauri::command]
async fn get_started(name: &str) -> Result<String, String> {

    let node = DcNodeInit::new().await;


    if false {
        Err("Name should not contain spaces".to_string())
    } else {
        Ok(
            encode_upper(node.address)
        )
    }
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_started])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

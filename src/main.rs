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
    pubkey: String
}

static mut CRED: Vec<String> = vec![];

#[tauri::command]
async fn get_started() -> Result<String, String> {

    let node = DcNodeInit::new().await;
    
    unsafe{
        CRED.push(encode_upper(node.address.clone()));
        CRED.push(encode_upper(node.pub_key_vec.clone()));
    }

    if false {
        Err("ERROROROROROR".to_string())
    } else {
        Ok(
            encode_upper(node.address)
        )
    }
}

#[tauri::command]
async fn get_pub() -> Result<String, String> {

    if false {
        Err("ERROROROROROR".to_string())
    } else {
        Ok(
            unsafe{
                CRED[1].clone()
            }
        )
    }
}
fn main() {

    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_started,get_pub])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}

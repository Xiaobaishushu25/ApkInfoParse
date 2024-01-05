// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;

use app::{apk_info::ApkParsedInfo, manifest_parser::parser};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_by_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
// fn parse_by_path(path:&str) -> apk_info::ApkParsedInfo{
// fn parse_by_path(path:&str) -> Response{
// fn parse_by_path(path:&str) -> AppResponse{
fn parse_by_path(path: &str) -> Result<ApkParsedInfo, ErrRes> {
    println!("{}", path);
    // return match parser::parse(&PathBuf::from_str(path).unwrap()).await{
    return match parser::parse(&PathBuf::from_str(path).unwrap()) {
        None => {
            Err(ErrRes {
                code: 200,
                msg: "parse failure".to_string(),
            })
            // AppResponse(Err(ErrRes{
            //         code:200,
            //         msg:"parse failure".to_string()
            //     }))
            // Response::Err(ErrRes{
            //     code:200,
            //     msg:"parse failure".to_string()
            // })
        }
        Some(apkinfo) => {
            // Response::Ok(apkinfo)
            // AppResponse(Ok(apkinfo))
            Ok(apkinfo)
            // Ok(apkinfo)
        }
    };
    // return if let Some(info) = parser::parse(&PathBuf::from_str("E:\\待处理文件\\虎扑.apk.1").unwrap()).await {
    //   Response::Ok(Res{
    //       code:200,
    //       data:info,
    //       msg:"success".to_string(),
    //   })
    // }else {
    //     Response::Err(ErrRes{
    //         code:200,
    //         msg:"parse failure".to_string()
    //     }
    //     )
    // }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AppResponse(pub AppResult);
// pub type AppResult = Result<ApkParsedInfo,Box<dyn std::error::Error>>;
pub type AppResult = Result<ApkParsedInfo, ErrRes>;
#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    /// Contains the success value
    Ok(ApkParsedInfo),

    /// Contains the error value
    Err(ErrRes),
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Res<T> {
    pub code: i32,
    pub data: T,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ErrRes {
    pub code: i32,
    pub msg: String,
}

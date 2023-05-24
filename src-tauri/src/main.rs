// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str,age:&str) -> String {
    println!("greet");
    println!("name {}", name);
    println!("age {}", age);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::process::{Command, Stdio};
use tokio::process::Command as AsyncCommand;


#[tauri::command]
async fn start_minio() -> Result<String, String> {
    println!("start_minio");
    let status = AsyncCommand::new("cmd")
        .current_dir("C:\\SoftWare\\minio")
        .args(&["/C", "minio.exe", "server", "C:\\SoftWare\\minio\\Data", "--console-address", ":9010", "--address", ":9090"])
        .status()
        .await
        .map_err(|e| e.to_string())?;
    println!("status.success {}", status.success());
    if status.success() {
        Ok("MinIO started successfully".into())
    } else {
        Err("MinIO failed to start".into())
    }
}


#[tauri::command]
async fn start_server(path: String,cmdStr: Vec<String>) -> Result<String, String> {
    println!("start_server");

    println!("The path is {}", path);
    println!("The cmd_str  is {:?}", cmdStr);

    let status = AsyncCommand::new("cmd")
        .current_dir(path)
        .args(cmdStr)
        .status()
        .await
        .map_err(|e| e.to_string())?;
    println!("status.success {}", status.success());
    if status.success() {
        Ok("MinIO started successfully".into())
    } else {
        Err("MinIO failed to start".into())
    }
}




use tauri::Manager;

fn main() {
    tauri::Builder::default()
        //新增关闭提示的逻辑
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
                {
                    let window = app.get_window("main").unwrap();
                    window.open_devtools();
                    window.close_devtools();
                }
            Ok(())
        })
        .on_window_event(|event| {
            match event.event() {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    //阻止默认关闭
                    api.prevent_close();

                    let window = event.window().clone();
                    tauri::api::dialog::confirm(Some(&event.window()), "关闭应用", "确定关闭当前应用?", move |answer| {
                        if answer {
                            let _result = window.close();//直接接收一下即可，_表示让浏览器忽略未使用的变量
                        }
                    })
                }
                _ => {}//todo
            }
        })
        .invoke_handler(tauri::generate_handler![greet,start_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



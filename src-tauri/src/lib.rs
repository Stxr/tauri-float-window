use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn set_always_on_top(window: tauri::Window, always_on_top: bool) {
    let _ = window.set_always_on_top(always_on_top);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let initial_size = window.inner_size().unwrap();
            let aspect_ratio = initial_size.width as f64 / initial_size.height as f64;
            
            window.clone().on_window_event(move |event| {
                if let tauri::WindowEvent::Resized { .. } = event {
                    // 获取当前窗口大小
                    if let Ok(size) = window.inner_size() {
                        // let current_width = size.width as f64;
                        // let target_height = current_width / aspect_ratio;
                        // let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                        //     width: current_width as u32,
                        //     height: target_height as u32,
                        // }));
                        
                        let current_width = size.width as f64;
                        let current_height = size.height as f64;
                        let current_ratio = current_width / current_height;
                        
                        // 根据拖动方向决定是调整宽度还是高度
                        if current_ratio > aspect_ratio {
                            // 宽度过大，调整宽度
                            let new_width = (current_height * aspect_ratio) as u32;
                            let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                                width: new_width,
                                height: size.height,
                            }));
                        } else if current_ratio < aspect_ratio {
                            // 高度过大，调整高度
                            let new_height = (current_width / aspect_ratio) as u32;
                            let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                                width: size.width,
                                height: new_height,
                            }));
                        }
                    }
                }
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, set_always_on_top])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

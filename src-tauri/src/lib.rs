use std::{fs, path::PathBuf};
use tauri::{
    menu::{Menu, MenuItem, Submenu},
    Manager, WebviewUrl, WebviewWindowBuilder,
};

const URL_CONFIG_FILE: &str = "url_config.json"; // stored under app config dir

fn url_config_path(app: &tauri::AppHandle) -> Option<PathBuf> {
    app.path()
        .app_config_dir()
        .ok()
        .map(|dir| dir.join(URL_CONFIG_FILE))
}

fn read_saved_url(app: &tauri::AppHandle) -> Option<String> {
    let path = url_config_path(app)?;
    let data = fs::read_to_string(path).ok()?;
    let v: serde_json::Value = serde_json::from_str(&data).ok()?;
    v.get("startup_url")
        .and_then(|s| s.as_str())
        .map(|s| s.to_string())
}

fn write_saved_url(app: &tauri::AppHandle, url: &str) -> Result<(), String> {
    let Some(path) = url_config_path(app) else {
        return Err("Failed to resolve app config dir".into());
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let body = serde_json::json!({"startup_url": url});
    fs::write(
        path,
        serde_json::to_vec_pretty(&body).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn set_always_on_top(window: tauri::Window, always_on_top: bool) {
    let _ = window.set_always_on_top(always_on_top);
}

#[tauri::command]
fn start_dragging(window: tauri::Window) {
    let _ = window.start_dragging();
}

#[tauri::command]
fn minimize_window(window: tauri::Window) {
    let _ = window.minimize();
}

#[tauri::command]
fn toggle_maximize_window(window: tauri::Window) {
    let _ = window.is_maximized().map(|is_maxed| {
        if is_maxed {
            window.unmaximize()
        } else {
            window.maximize()
        }
    });
}

#[tauri::command]
fn close_window(window: tauri::Window) {
    let _ = window.close();
}

// Save URL to app config file
#[tauri::command]
fn save_url(app: tauri::AppHandle, url: String) -> Result<(), String> {
    write_saved_url(&app, &url)
}

// Read URL from app config file
#[tauri::command]
fn get_saved_url(app: tauri::AppHandle) -> Option<String> {
    read_saved_url(&app)
}

// Navigate main window to provided URL
#[tauri::command]
fn navigate_to(app: tauri::AppHandle, url: String) -> Result<(), String> {
    if let Some(main) = app.get_webview_window("main") {
        let js = format!(
            "window.location.href = '{}';",
            url.replace('\\', "\\\\").replace('"', "\\\"")
        );
        main.eval(&js).map_err(|e| e.to_string())
    } else {
        Err("main window not found".into())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // App menu with a Settings submenu for URL management
        .setup(|app| {
            // Build menu
            // Add macOS-style shortcut: Cmd+, opens the settings window
            // Use a cross-platform accelerator so Windows/Linux can use Ctrl+, as well.
            // Accelerator grammar follows the tao/accelerator crate (e.g. "CmdOrCtrl+Comma").
            let set_url = MenuItem::with_id(
                app,
                "menu-set-url",
                "设置加载 URL...",
                true,
                Some("CmdOrCtrl+Comma"),
            )?;
            let use_saved = MenuItem::with_id(
                app,
                "menu-use-saved-url",
                "加载已保存 URL",
                true,
                None::<&str>,
            )?;
            let clear_saved = MenuItem::with_id(
                app,
                "menu-clear-saved-url",
                "清除已保存 URL",
                true,
                None::<&str>,
            )?;
            let edit_cfg = MenuItem::with_id(
                app,
                "menu-edit-url-config",
                "编辑 URL 配置文件...",
                true,
                None::<&str>,
            )?;
            let submenu = Submenu::with_items(
                app,
                "设置",
                true,
                &[&set_url, &use_saved, &clear_saved, &edit_cfg],
            )?;
            let menu = Menu::with_items(app, &[&submenu])?;
            app.set_menu(menu)?;

            // Enforce aspect ratio on main window resize (existing logic)
            let window = app.get_webview_window("main").unwrap();
            let initial_size = window.inner_size().unwrap();
            let aspect_ratio = initial_size.width as f64 / initial_size.height as f64;

            window.clone().on_window_event(move |event| {
                if let tauri::WindowEvent::Resized { .. } = event {
                    if let Ok(size) = window.inner_size() {
                        let current_width = size.width as f64;
                        let current_height = size.height as f64;
                        let current_ratio = current_width / current_height;

                        if current_ratio > aspect_ratio {
                            let new_width = (current_height * aspect_ratio) as u32;
                            let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                                width: new_width,
                                height: size.height,
                            }));
                        } else if current_ratio < aspect_ratio {
                            let new_height = (current_width / aspect_ratio) as u32;
                            let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                                width: size.width,
                                height: new_height,
                            }));
                        }
                    }
                }
            });

            // On dev builds, if a saved URL exists, navigate to it
            #[cfg(debug_assertions)]
            {
                let app_handle = app.handle();
                let url_to_load = read_saved_url(app_handle)
                    .or_else(|| Some(String::from("http://127.0.0.1:1420")));
                if let Some(u) = url_to_load {
                    if let Some(main) = app.get_webview_window("main") {
                        let _ = main.eval(format!(
                            "window.location.href = '{}';",
                            u.replace('\\', "\\\\").replace('"', "\\\"")
                        ));
                    }
                }
            }

            Ok(())
        })
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "menu-set-url" => {
                    // Open a small local settings window (bundled page) to input URL
                    if app.get_webview_window("settings").is_none() {
                        let _ = WebviewWindowBuilder::new(
                            app,
                            "settings",
                            WebviewUrl::App("/settings.html".into()),
                        )
                        .title("设置加载 URL")
                        .inner_size(520.0, 160.0)
                        .resizable(false)
                        .build();
                    } else if let Some(w) = app.get_webview_window("settings") {
                        let _ = w.set_focus();
                    }
                }
                "menu-use-saved-url" => {
                    if let Some(u) = read_saved_url(app) {
                        if let Some(main) = app.get_webview_window("main") {
                            let _ = main.eval(format!(
                                "window.location.href = '{}';",
                                u.replace('\\', "\\\\").replace('"', "\\\"")
                            ));
                        }
                    }
                }
                "menu-clear-saved-url" => {
                    if let Some(path) = url_config_path(app) {
                        let _ = fs::remove_file(path);
                    }
                }
                "menu-edit-url-config" => {
                    if let Some(path) = url_config_path(app) {
                        if !path.exists() {
                            let _ = write_saved_url(app, "http://127.0.0.1:1420");
                        }
                        #[cfg(target_os = "macos")]
                        {
                            let _ = std::process::Command::new("open").arg(&path).spawn();
                        }
                        #[cfg(target_os = "windows")]
                        {
                            let _ = std::process::Command::new("cmd")
                                .args(["/C", "start", path.to_string_lossy().as_ref()])
                                .spawn();
                        }
                        #[cfg(target_os = "linux")]
                        {
                            let _ = std::process::Command::new("xdg-open").arg(&path).spawn();
                        }
                    }
                }
                _ => {}
            }
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            set_always_on_top,
            start_dragging,
            minimize_window,
            toggle_maximize_window,
            close_window,
            save_url,
            get_saved_url,
            navigate_to
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

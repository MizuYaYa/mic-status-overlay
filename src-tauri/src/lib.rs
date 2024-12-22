use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use windows::{
    core::Result,
    Win32::{
        Media::Audio::{
            eCapture, eConsole, Endpoints::IAudioEndpointVolume, IMMDeviceEnumerator,
            MMDeviceEnumerator,
        },
        System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
        UI::WindowsAndMessaging::{
            SetWindowLongA, GWL_EXSTYLE, WS_EX_APPWINDOW, WS_EX_COMPOSITED, WS_EX_LAYERED,
            WS_EX_TOPMOST, WS_EX_TRANSPARENT,
        },
    },
};

#[tauri::command]
fn get_mic_status() -> String {
    match get_default_mic_mute_status() {
        Ok(status) => format!("{}", status),
        Err(err) => format!("Error: {}", err),
    }
}

fn get_default_mic_mute_status() -> Result<bool> {
    unsafe {
        let devicenum: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER)?;

        let defaultdevice = devicenum.GetDefaultAudioEndpoint(eCapture, eConsole)?;

        let endpointval: IAudioEndpointVolume =
            defaultdevice.Activate(CLSCTX_INPROC_SERVER, None)?;

        let is_muted = endpointval.GetMute()?.as_bool();

        Ok(is_muted)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .setup(move |app| {
            let window = app.get_webview_window("main").unwrap();
            window
                .set_size(tauri::Size::Physical(tauri::PhysicalSize {
                    width: 1920,
                    height: 70,
                }))
                .unwrap();
            let hwnd = window.hwnd().unwrap().0;
            let _pre_val;
            let hwnd = windows::Win32::Foundation::HWND(hwnd as isize);
            unsafe {
                let nindex = GWL_EXSTYLE;
                let style = WS_EX_APPWINDOW
                    | WS_EX_COMPOSITED
                    | WS_EX_LAYERED
                    | WS_EX_TRANSPARENT
                    | WS_EX_TOPMOST;
                _pre_val = SetWindowLongA(hwnd, nindex, style.0 as i32);
            };

            let separator = PredefinedMenuItem::separator(app)?;
            let quit_i = MenuItem::with_id(app, "quit", "終了", true, None::<&str>)?;
            let version = MenuItem::new(
                app,
                format!("v{}", app.package_info().version.to_string()),
                false,
                None::<&str>,
            )?;

            let menu = Menu::with_items(app, &[&version, &separator, &quit_i])?;
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip(app.package_info().name.to_string())
                .build(app)?;

            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            use tauri_plugin_notification::NotificationExt;
            app.notification()
            .builder()
            .title("動作中")
            .body(format!("{}は既にバックグラウンド動作しています", app.package_info().name.to_string()))
            .show()
            .unwrap();

        }))
        .invoke_handler(tauri::generate_handler![get_mic_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{App, Error, Manager};
use tauri_nspanel::WebviewWindowExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_nspanel::init())
        .setup(|app| {
            setup_tray(app)?;
            setup_hotkey(app)?;

            let window = app.get_webview_window("main").unwrap();
            let _panel = window.to_panel().unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_tray(app: &App) -> Result<(), Error> {
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit])?;

    let _ = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                println!("quit menu item was clicked");
                app.exit(0);
            }
            _ => {}
        })
        .build(app);

    Ok(())
}

pub fn setup_hotkey(app: &App) -> Result<(), tauri_plugin_global_shortcut::Error> {
    let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);

    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |_app, shortcut, event| {
                println!("{:?}", shortcut);
                if shortcut == &ctrl_n_shortcut {
                    match event.state() {
                        ShortcutState::Pressed => {
                            println!("Ctrl-N Pressed!");
                        }
                        ShortcutState::Released => {
                            println!("Ctrl-N Released!");
                        }
                    }
                }
            })
            .build(),
    )?;

    app.global_shortcut().register(ctrl_n_shortcut)?;

    Ok(())
}

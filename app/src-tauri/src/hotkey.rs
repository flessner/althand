use tauri::App;
use tauri_plugin_global_shortcut::{
    Code, Error, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
};

pub fn setup(app: &App) -> Result<(), Error> {
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

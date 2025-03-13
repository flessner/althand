use tauri::{Emitter, Manager, Runtime, WebviewWindow};
use tauri_nspanel::{
    cocoa::appkit::{NSMainMenuWindowLevel, NSWindowCollectionBehavior},
    objc::{msg_send, sel, sel_impl},
    panel_delegate, Panel, WebviewWindowExt as PanelWebviewWindowExt,
};

// source: https://github.com/ahkohd/tauri-macos-spotlight-example/blob/v2/src-tauri/src/window.rs
pub trait WebviewWindowExt {
    fn to_spotlight_panel(&self) -> tauri::Result<Panel>;
}

impl<R: Runtime> WebviewWindowExt for WebviewWindow<R> {
    fn to_spotlight_panel(&self) -> tauri::Result<Panel> {
        let panel = self.to_panel().expect("Error converting window to panel");

        panel.set_level(NSMainMenuWindowLevel + 1);

        panel.set_collection_behaviour(
            NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary,
        );

        #[allow(non_upper_case_globals)]
        const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;

        panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel);

        let panel_delegate = panel_delegate!(SpotlightPanelDelegate {
            window_did_resign_key,
            window_did_become_key
        });

        let app_handle = self.app_handle().clone();

        let label = self.label().to_string();

        panel_delegate.set_listener(Box::new(move |delegate_name: String| {
            match delegate_name.as_str() {
                "window_did_become_key" => {
                    let _ = app_handle.emit(format!("{}_panel_did_become_key", label).as_str(), ());
                }
                "window_did_resign_key" => {
                    let _ = app_handle.emit(format!("{}_panel_did_resign_key", label).as_str(), ());
                }
                _ => (),
            }
        }));

        panel.set_delegate(panel_delegate);

        Ok(panel)
    }
}

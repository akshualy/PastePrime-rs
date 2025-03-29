use auto_launch::AutoLaunch;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use trayicon::TrayIcon;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use crate::clipboard_typer::ClipboardTyper;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum UserEvent {
    ClickTrayIcon,
    AutoStartItem,
    ExitItem,
    GlobalHotKeyEvent(GlobalHotKeyEvent),
}

pub struct PastePrimeApp {
    _hotkey_manager: GlobalHotKeyManager,  // We need to store the manager to keep it alive
    hotkey_id: u32,
    clipboard_typer: ClipboardTyper,
    auto_launch: AutoLaunch,
    tray_icon: TrayIcon<UserEvent>,
}

impl PastePrimeApp {
    pub fn new(
        hotkey_manager: GlobalHotKeyManager,
        hotkey_id: u32,
        clipboard_typer: ClipboardTyper,
        auto_launch: AutoLaunch,
        tray_icon: TrayIcon<UserEvent>,
    ) -> Self {
        Self {
            _hotkey_manager: hotkey_manager,
            hotkey_id,
            clipboard_typer,
            auto_launch,
            tray_icon,
        }
    }

    fn handle_auto_start_toggle(&mut self) {
        let auto_launch_enabled = self.auto_launch.is_enabled().unwrap();
        if auto_launch_enabled {
            self.auto_launch.disable().unwrap();
        } else {
            self.auto_launch.enable().unwrap();
        }
        let _ = self.tray_icon
            .set_menu_item_checkable(UserEvent::AutoStartItem, !auto_launch_enabled);
    }
}

impl ApplicationHandler<UserEvent> for PastePrimeApp {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::ClickTrayIcon => {
                self.tray_icon.show_menu().unwrap();
            }
            UserEvent::ExitItem => event_loop.exit(),
            UserEvent::AutoStartItem => self.handle_auto_start_toggle(),
            UserEvent::GlobalHotKeyEvent(event) => {
                if event.id == self.hotkey_id && event.state == HotKeyState::Pressed {
                    self.clipboard_typer.type_clipboard_content();
                }
            }
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: WindowEvent,
    ) {
    }
} 
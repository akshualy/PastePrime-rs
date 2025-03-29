// Hide the console window on Windows
#![windows_subsystem = "windows"]

mod clipboard_typer;
mod app;

use std::env;
use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use trayicon::{Icon, MenuBuilder, TrayIcon, TrayIconBuilder};
use winit::event_loop::EventLoop;
use clipboard_typer::ClipboardTyper;
use app::{PastePrimeApp, UserEvent};

fn setup_auto_launch() -> AutoLaunch {
    AutoLaunchBuilder::new()
        .set_app_name("Paste Prime")
        .set_app_path(env::current_exe().unwrap().to_str().unwrap())
        .set_use_launch_agent(true)
        .build()
        .unwrap()
}

fn setup_hotkey(event_loop: &EventLoop<UserEvent>) -> (GlobalHotKeyManager, u32) {
    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyV);
    manager.register(hotkey).unwrap();
    let hotkey_id = hotkey.id();

    let proxy = event_loop.create_proxy();
    GlobalHotKeyEvent::set_event_handler(Some(move |event| {
        proxy.send_event(UserEvent::GlobalHotKeyEvent(event)).unwrap();
    }));

    (manager, hotkey_id)
}

fn setup_tray_icon(
    event_loop: &EventLoop<UserEvent>, auto_launch: &AutoLaunch
) -> TrayIcon<UserEvent> {
    let icon_buffer = include_bytes!("../pasteprime.ico");
    let icon = Icon::from_buffer(icon_buffer, Some(32), Some(32)).unwrap();

    let proxy = event_loop.create_proxy();
    TrayIconBuilder::new()
        .sender(move |event: &UserEvent| {
            let _ = proxy.send_event(event.clone());
        })
        .icon(icon)
        .tooltip("Paste Prime")
        .on_click(UserEvent::ClickTrayIcon)
        .on_right_click(UserEvent::ClickTrayIcon)
        .menu(create_tray_menu(auto_launch))
        .build()
        .unwrap()
}

fn create_tray_menu(auto_launch: &AutoLaunch) -> MenuBuilder<UserEvent> {
    MenuBuilder::new()
        .checkable(
            "Start with Windows",
            auto_launch.is_enabled().unwrap(),
            UserEvent::AutoStartItem,
        )
        .item("Exit", UserEvent::ExitItem)
}

fn main() {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
    let (hotkey_manager, hotkey_id) = setup_hotkey(&event_loop);
    let clipboard_typer = ClipboardTyper::new().unwrap();
    let auto_launch = setup_auto_launch();
    let tray_icon = setup_tray_icon(&event_loop, &auto_launch);

    let mut app = PastePrimeApp::new(
        hotkey_manager,
        hotkey_id,
        clipboard_typer,
        auto_launch,
        tray_icon,
    );

    event_loop.run_app(&mut app).unwrap();
}

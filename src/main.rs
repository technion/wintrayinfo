#![deny(unsafe_code)]
#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use ipconfig::get_adapters;
use nwd::NwgUi;
use nwg::NativeUi;
use std::format;

// Bundling the icon means we don't need to ship the file separately
static ICON_DATA: &[u8] = include_bytes!("../logo.ico");

#[derive(Default, NwgUi)]
pub struct SystemTray {
    #[nwg_control]
    window: nwg::MessageWindow,

    #[nwg_resource(source_bin: Some(ICON_DATA))]
    icon: nwg::Icon,

    #[nwg_control(icon: Some(&data.icon), tip: Some("Desktop Info"))]
    #[nwg_events(MousePressLeftUp: [SystemTray::show_menu], OnContextMenu: [SystemTray::show_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "Display Information")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::hello2])]
    tray_item2: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "Exit")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::exit])]
    tray_item3: nwg::MenuItem,
}

fn get_ip_list() -> String {
    let mut iplist = Vec::new();

    for adapter in get_adapters().unwrap() {
        for ipaddress in adapter.ip_addresses() {
            let ipstr = ipaddress.to_string();
            if ipstr.starts_with("192.") || ipstr.starts_with("10.") || ipstr.starts_with("172.") {
                iplist.push(ipstr);
            }
        }
    }
    iplist.join(", ")
}

impl SystemTray {
    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn hello2(&self) {
        let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;

        let infostring = format!(
            "Username: {}\nHostname: {}\n",
            whoami::username(),
            whoami::hostname(),
        );
        self.tray.show(
            &get_ip_list(),
            Some(&infostring),
            Some(flags),
            Some(&self.icon),
        );
    }

    fn exit(_: &Self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    println!("Launched Wintrayinfo!");
    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = SystemTray::build_ui(SystemTray::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}

#[cfg(test)]
mod tests {
    use super::get_ip_list;
    #[test]
    fn obtains_an_ip() {
        // This can't test much more than "actually gets data and doesn't crash"
        assert!(get_ip_list().len() > 1)
    }
}

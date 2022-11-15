#![deny(unsafe_code)]
#![windows_subsystem = "windows"] 

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use ipconfig::get_adapters;
use nwd::NwgUi;
use nwg::NativeUi;
use std::format;

#[derive(Default, NwgUi)]
pub struct SystemTray {
    #[nwg_control]
    window: nwg::MessageWindow,

    #[nwg_resource(source_file: Some("./logo.ico"))]
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

impl SystemTray {
    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn hello2(&self) {
        let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
        let mut iplist = Vec::new();

        for adapter in get_adapters().unwrap() {
            for ipaddress in adapter.ip_addresses() {
                let ipstr = ipaddress.to_string();
                if ipstr.starts_with("192.") || ipstr.starts_with("10.") || ipstr.starts_with("172.") {
                    iplist.push(ipstr);
                }
            }
        }
        let infostring = format!(
            "Username: {}\nHostname: {}\n",
            whoami::username(),
            whoami::hostname(),           
        );
        self.tray.show(
            &iplist.join(", "),
            Some(&infostring),
            Some(flags),
            Some(&self.icon),
        );
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    println!("Launched Wintrayinfo!");
    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = SystemTray::build_ui(SystemTray::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}

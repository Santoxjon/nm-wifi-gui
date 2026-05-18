use std::cell::RefCell;
use std::rc::Rc;
use std::thread;

use adw::prelude::*;
use gtk::glib::{self, ControlFlow};
use gtk::prelude::*;

use crate::backend::nmcli::NmcliBackend;
use crate::backend::WifiBackend;
use crate::models::WifiNetwork;

#[derive(Clone)]
struct UiState {
    backend: NmcliBackend,
    networks: Rc<RefCell<Vec<WifiNetwork>>>,
    listbox: gtk::ListBox,
    status_label: gtk::Label,
    error_label: gtk::Label,
    window: adw::ApplicationWindow,
}

pub fn build_ui(app: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("WiFi Manager")
        .default_width(420)
        .default_height(560)
        .build();

    let header = adw::HeaderBar::builder()
        .title_widget(&gtk::Label::new(Some("WiFi")))
        .show_end_title_buttons(true)
        .build();

    let status_label = gtk::Label::builder()
        .xalign(0.0)
        .label("Scanning...")
        .margin_top(8)
        .margin_bottom(4)
        .build();

    let error_label = gtk::Label::builder()
        .xalign(0.0)
        .wrap(true)
        .css_classes(["error"])
        .build();

    let listbox = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();

    let scroller = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vexpand(true)
        .child(&listbox)
        .build();

    let refresh_button = gtk::Button::builder().label("Refresh").build();

    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(8)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    content.append(&status_label);
    content.append(&error_label);
    content.append(&scroller);
    content.append(&refresh_button);

    let root = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    root.append(&header);
    root.append(&content);

    window.set_content(Some(&root));

    let state = UiState {
        backend: NmcliBackend,
        networks: Rc::new(RefCell::new(Vec::new())),
        listbox: listbox.clone(),
        status_label: status_label.clone(),
        error_label: error_label.clone(),
        window: window.clone(),
    };

    {
        let state = state.clone();
        refresh_button.connect_clicked(move |_| {
            refresh_networks(&state);
        });
    }

    {
        let state = state.clone();
        listbox.connect_row_activated(move |_, row| {
            let index = row.index();
            if index < 0 {
                return;
            }

            let maybe_network = state.networks.borrow().get(index as usize).cloned();
            if let Some(network) = maybe_network {
                connect_to_network(&state, network);
            }
        });
    }

    {
        let state = state.clone();
        glib::timeout_add_seconds_local(5, move || {
            refresh_networks(&state);
            ControlFlow::Continue
        });
    }

    refresh_networks(&state);
    window.present();
}

fn refresh_networks(state: &UiState) {
    state.error_label.set_text("");

    let backend = state.backend.clone();
    let (sender, receiver) = std::sync::mpsc::channel::<anyhow::Result<Vec<WifiNetwork>>>();

    thread::spawn(move || {
        let result = backend.scan_networks();
        let _ = sender.send(result);
    });

    let state = state.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        match receiver.try_recv() {
            Ok(Ok(networks)) => {
                *state.networks.borrow_mut() = networks.clone();
                rebuild_list(&state.listbox, &networks);
                update_connected_status(&state.status_label, &networks);
                ControlFlow::Break
            }
            Ok(Err(err)) => {
                state.error_label.set_text(&format!("Scan failed: {err}"));
                ControlFlow::Break
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => ControlFlow::Continue,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                state.error_label.set_text("Scan failed: worker disconnected");
                ControlFlow::Break
            }
        }
    });
}

fn rebuild_list(listbox: &gtk::ListBox, networks: &[WifiNetwork]) {
    while let Some(child) = listbox.first_child() {
        listbox.remove(&child);
    }

    for network in networks {
        let row = gtk::ListBoxRow::new();
        row.set_activatable(true);
        row.set_selectable(false);

        let outer = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(2)
            .margin_top(8)
            .margin_bottom(8)
            .margin_start(8)
            .margin_end(8)
            .build();

        let top = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(8)
            .build();

        let ssid = if network.ssid.is_empty() {
            "<hidden>".to_string()
        } else {
            network.ssid.clone()
        };

        let mut title = ssid;
        if network.connected {
            title.push_str("  (connected)");
        } else if network.known {
            title.push_str("  (saved)");
        }

        let ssid_label = gtk::Label::builder()
            .label(&title)
            .xalign(0.0)
            .hexpand(true)
            .build();

        let signal_label = gtk::Label::builder()
            .label(format!("{}%", network.signal))
            .xalign(1.0)
            .build();

        let sec_label = gtk::Label::builder()
            .label(format!("Security: {}", network.security))
            .xalign(0.0)
            .css_classes(["dim-label"])
            .build();

        top.append(&ssid_label);
        top.append(&signal_label);
        outer.append(&top);
        outer.append(&sec_label);

        row.set_child(Some(&outer));
        listbox.append(&row);
    }
}

fn update_connected_status(status_label: &gtk::Label, networks: &[WifiNetwork]) {
    if let Some(current) = networks.iter().find(|n| n.connected) {
        status_label.set_text(&format!(
            "Connected: {} ({}%)",
            current.ssid, current.signal
        ));
    } else {
        status_label.set_text("Not connected to WiFi");
    }
}

fn connect_to_network(state: &UiState, network: WifiNetwork) {
    state.error_label.set_text("");

    if network.known || !network.requires_password() {
        run_connect_no_password(state, &network.ssid);
    } else {
        prompt_password_and_connect(state, &network.ssid);
    }
}

fn run_connect_no_password(state: &UiState, ssid: &str) {
    let backend = state.backend.clone();
    let ssid_owned = ssid.to_string();
    let (sender, receiver) = std::sync::mpsc::channel::<anyhow::Result<()>>();

    thread::spawn(move || {
        let result = backend.connect_known(&ssid_owned);
        let _ = sender.send(result);
    });

    let state = state.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        match receiver.try_recv() {
            Ok(Ok(())) => {
                refresh_networks(&state);
                ControlFlow::Break
            }
            Ok(Err(err)) => {
                state
                    .error_label
                    .set_text(&format!("Connect failed: {err}"));
                ControlFlow::Break
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => ControlFlow::Continue,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                state
                    .error_label
                    .set_text("Connect failed: worker disconnected");
                ControlFlow::Break
            }
        }
    });
}

fn prompt_password_and_connect(state: &UiState, ssid: &str) {
    let dialog = gtk::Dialog::builder()
        .transient_for(&state.window)
        .modal(true)
        .title(format!("Connect to {ssid}"))
        .build();

    dialog.add_button("Cancel", gtk::ResponseType::Cancel);
    dialog.add_button("Connect", gtk::ResponseType::Accept);

    let content = dialog.content_area();
    content.set_spacing(0);

    let dialog_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(10)
        .margin_top(16)
        .margin_bottom(16)
        .margin_start(16)
        .margin_end(16)
        .build();

    let hint_label = gtk::Label::builder()
        .label(format!("Enter password for {ssid}"))
        .xalign(0.0)
        .wrap(true)
        .css_classes(["dim-label"])
        .build();

    let password_entry = gtk::PasswordEntry::builder()
        .placeholder_text("WiFi password")
        .show_peek_icon(true)
        .hexpand(true)
        .build();

    dialog_box.append(&hint_label);
    dialog_box.append(&password_entry);
    content.append(&dialog_box);

    let state = state.clone();
    let ssid_owned = ssid.to_string();
    dialog.connect_response(move |d, response| {
        if response == gtk::ResponseType::Accept {
            let password = password_entry.text().to_string();
            if !password.is_empty() {
                run_connect_with_password(&state, &ssid_owned, &password);
            } else {
                state.error_label.set_text("Password cannot be empty");
            }
        }

        d.close();
    });

    dialog.present();
}

fn run_connect_with_password(state: &UiState, ssid: &str, password: &str) {
    let backend = state.backend.clone();
    let ssid_owned = ssid.to_string();
    let password_owned = password.to_string();
    let (sender, receiver) = std::sync::mpsc::channel::<anyhow::Result<()>>();

    thread::spawn(move || {
        let result = backend.connect_with_password(&ssid_owned, &password_owned);
        let _ = sender.send(result);
    });

    let state = state.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        match receiver.try_recv() {
            Ok(Ok(())) => {
                refresh_networks(&state);
                ControlFlow::Break
            }
            Ok(Err(err)) => {
                state
                    .error_label
                    .set_text(&format!("Connect failed: {err}"));
                ControlFlow::Break
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => ControlFlow::Continue,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                state
                    .error_label
                    .set_text("Connect failed: worker disconnected");
                ControlFlow::Break
            }
        }
    });
}

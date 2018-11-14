#![allow(deprecated)]
extern crate gdk;
extern crate gdk_pixbuf;
extern crate glib;
extern crate gtk;
extern crate regex;

use gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use gtk::{Menu, MenuItem, Orientation, Scale, StatusIcon, Window, WindowType};



mod pulse;

const MAX: f64 = 65536.0;


fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let path = "/usr/share/icons/Apricity Icons/devices/symbolic/audio-headphones-symbolic.svg";
    let pix = Pixbuf::new_from_file_at_size(path, 32, 32).unwrap();
    let icon = StatusIcon::new_from_pixbuf(&pix);
    let menu = Menu::new();
    let volume_window = Window::new(WindowType::Toplevel);
    let volume_control = Scale::new_with_range(Orientation::Vertical, 0.0, 100.0, 5.0);
    volume_control.set_draw_value(false);
    volume_control.set_inverted(true);
    volume_window.add(&volume_control);
    volume_window.set_skip_taskbar_hint(true);
    volume_window.set_decorated(false);
    volume_window.set_default_size(28, 120);

    build_menu(&menu);

    volume_window.connect_focus_out_event(|window, e| {
        window.hide();
        Inhibit(false)
    });
    icon.connect_popup_menu(move |icon, btn, time| {
        menu.show_all();
        menu.popup_easy(btn, time);
    });
    icon.connect_activate(move |icon| {
        match icon.get_geometry() {
            Some((_screen, rect, _orient)) => volume_window.move_(rect.x, rect.y),
            None => (),
        }
        let (idx, vol) = pulse::get_volume();
        volume_control.set_value(vol.parse::<f64>().unwrap() / MAX * 100.0);
        volume_control.connect_value_changed(volume_changed);
        volume_window.show_all();
    });

    gtk::main();
}

fn build_menu(menu: &Menu) {
    let names = pulse::get_sinks();
    for sink in names {
        let item = MenuItem::new_with_label(&sink.name);
        item.connect_select(move |_| item_clicked(&sink));
        menu.append(&item);
    }
}
fn item_clicked(item: &pulse::Device) {
    pulse::exec(&format!("pacmd set-default-sink {}", item.index));
    let sources = pulse::get_sources();
    for source in sources {
        pulse::exec(&format!("pacmd move-sink-input {} {}", source, item.index));
    }
}
fn volume_changed(volume_control: &Scale) {
  let (idx, _vol) = pulse::get_volume();
  let vol = (volume_control.get_value() / 100.0) * MAX;
  pulse::exec(&format!(r#"pacmd set-sink-volume {} {}"#, idx, vol as i64));
}


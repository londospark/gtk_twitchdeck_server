extern crate gtk;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 140);

    let layout = gtk::Box::new(gtk::Orientation::Vertical, 6);

    let button = Button::new_with_label("Click me on stream!");
    layout.add(&button);

    let another_button = Button::new_with_label("Click me too please!");
    layout.add(&another_button);

    window.add(&layout);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("Clicked!");
    });

    gtk::main();
}
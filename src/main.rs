extern crate actix_web;
extern crate ears;
extern crate gtk;

use ears::{AudioController, Sound};
use gtk::prelude::*;
use gtk::{Button, Window, WindowType};
use std::thread;

use actix_web::{server, App, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn play_sound(filename: &'static str) {
    thread::spawn(move || {
        let mut sound = Sound::new(filename).unwrap();
        sound.play();
        while sound.is_playing() {}
    });
}

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
        play_sound("train_horn.wav");
    });

    another_button.connect_clicked(|button| {
        println!("{:#?}", button);
    });

    thread::spawn(move || {
        server::new(|| App::new().resource("/", |r| r.f(index)))
            .bind("127.0.0.1:8087")
            .unwrap()
            .run();
    });

    gtk::main();
}

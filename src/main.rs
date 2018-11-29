#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate ears;
extern crate gtk;

use ears::{AudioController, Sound};
use gtk::prelude::*;
use gtk::{Button, Window, WindowType};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::sync::Arc;

mod webserver;
mod config;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let mut file = File::open("twitchdeck.toml").expect("File not found.");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot read file.");
    let config: config::Config = toml::from_str(&contents).expect("Cannot parse file");
    let thread_safe_config : Arc<config::Config> = Arc::new(config.clone());

    // println!("{:?}", thread_safe_config);

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 140);

    let layout = gtk::Box::new(gtk::Orientation::Vertical, 6);

    let mut sfx: HashMap<gtk::Button, String> = HashMap::new();

    for sound_effect in config.sounds {
        let button = Button::new_with_label(&sound_effect.label);
        sfx.insert(button, sound_effect.filename);
    }
    
    for (btn, filename) in sfx {
        let ref_file = Arc::new(filename);
        btn.connect_clicked(move |_| {
            let outer_clone = ref_file.clone();
            thread::spawn(move || {
                let clone_file = outer_clone.clone();
                let mut sound = Sound::new(&clone_file).unwrap();
                sound.play();
                while sound.is_playing() {}
            });
        });
        layout.add(&btn);
    }

    window.add(&layout);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    
    thread::spawn(move || {
        let web_server_config = thread_safe_config.clone();
        webserver::start_server(web_server_config);
    });

    gtk::main();
}

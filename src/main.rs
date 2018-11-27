#[macro_use]
extern crate serde_derive;
extern crate crossbeam;
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

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    sounds: Vec<SoundEffect>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SoundEffect {
    filename: String,
    label: String,
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let mut file = File::open("twitchdeck.toml").expect("File not found.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot read file.");
    let config: Config = toml::from_str(&contents).expect("Cannot parse file");

    println!("{:?}", config);

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
    /*
    button.connect_clicked(|_| {
        println!("Clicked!");
        play_sound(&"train_horn.wav".to_string());
    });

    another_button.connect_clicked(|button| {
        println!("{:#?}", button);
    });
*/
    //TODO: Does rust have a better process abstraction that we can use?
    thread::spawn(move || {
        webserver::start_server();
    });

    gtk::main();
}

extern crate actix_web;

use self::actix_web::{http, server, App, HttpRequest, Json};
use std::sync::Arc;
use std::thread;
use ears::{AudioController, Sound};

use config;

#[derive(Deserialize, Debug)]
struct SoundRequest {
    sound: String,
}

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn get_sounds(_req: &HttpRequest) -> &'static str {
    "SFX List"
}

fn play_sound(body: Json<SoundRequest>) -> &'static str {
    println!("{:?}", body);
    thread::spawn(move || {
        let clone_file = body.sound.clone();
        let mut sound = Sound::new(&clone_file).unwrap();
        sound.play();
        while sound.is_playing() {}
    });
    "OK"
}

pub fn start_server(_config: Arc<config::Config>) {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(index))
            .resource("/get_sounds", |r| r.f(get_sounds))
            .resource("/play_sound", |r| {
                r.method(http::Method::POST).with(play_sound)
            })
    }).bind("127.0.0.1:8087")
    .unwrap()
    .run();
}

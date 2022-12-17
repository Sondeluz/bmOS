use std::ops::Add;
use std::time;
use sfml::{
    graphics::{
        Color, Font, RenderTarget, RenderWindow,
        Sprite, Text, Texture, Transformable,
    },
    window::{ContextSettings, Event, Key, Style},
};
use crate::{audio_recording, audio_to_text};

pub fn run() {
    let width = 800;
    let height = 600;

    let face = Texture::from_file("assets/faces/angry/2.jpg").unwrap();
    let mut sprite = Sprite::new();
    sprite.set_texture(&face, true);
    sprite.set_position((0.0, 0.0));

    let font = Font::from_file(("assets/font.ttf")).unwrap();

    let context_settings = ContextSettings {
        antialiasing_level: 0,
        ..Default::default()
    };

    let mut window = RenderWindow::new(
        (width, height),
        "bmOS",
        Style::CLOSE,
        &context_settings,
    );

    window.set_vertical_sync_enabled(true);


    let segments = audio_to_text::audio_to_text_samples(
        audio_recording::record_audio(time::Duration::from_secs(20)));

    let mut sampled_voice_message = Text::default();
    sampled_voice_message.set_font(&font);
    sampled_voice_message.set_character_size(25);
    sampled_voice_message.set_position((width as f32 / 12.0, height as f32 / 2.0));

    let mut msg = String::from("You said...\n");
    for segment in segments {
        sampled_voice_message.set_fill_color(Color::WHITE);
        let words = segment.split(' ');
        let mut words_in_line = 0;
        for word in words {
            msg = msg.add(word);

            words_in_line += 1;
            if words_in_line == 4 {
                msg = msg.add("\n");
                words_in_line = 0;
            } else {
                msg = msg.add(" ");
            }
        }

    }
    sampled_voice_message.set_string(&msg);

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        window.draw(&sprite);
        window.draw(&sampled_voice_message);
        window.display();
    }
}


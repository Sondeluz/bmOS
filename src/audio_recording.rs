use sfml::{
    audio::{capture, SoundBufferRecorder}
};

use std::time;


// Records audio samples from the microphone for the given duration, and
// returns them as 16-bit mono samples with a 16KHz sample rate
pub fn record_audio(time: time::Duration) -> Vec<i16> {
    // Check that the device can capture audio
    assert!(
        capture::is_available(),
        "Sorry, audio capture is not supported by your system"
    );

    let sample_rate: u32 = 16000; // Whisper needs a sample rate of 16KHz
    let mut recorder = SoundBufferRecorder::new();

    // Audio capture is done in a separate thread,
    // so we can block the main thread while it is capturing
    println!("Recording...");
    recorder.start(sample_rate);
    std::thread::sleep(time);
    println!("Finished recording");
    recorder.stop();

    // Get the buffer containing the captured data
    let buffer = recorder.buffer();

    // Display captured sound information
    println!("Sound information :");
    println!(" {} seconds", buffer.duration().as_seconds());
    println!(" {} samples / sec", buffer.sample_rate());
    println!(" {} channels", buffer.channel_count());

    let mut samples = vec!();
    samples.extend_from_slice(buffer.samples());

    /*if !buffer.save_to_file("recorded_audio.wav".trim()) {
        eprintln!("Error saving buffer to {}!", "recorded_audio.wav".trim());
    }*/


    samples
}
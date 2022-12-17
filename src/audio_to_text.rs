use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};


pub fn run(audio_data : Vec<i16>) {
    let mut ctx = WhisperContext::new("models/ggml-base.bin").expect("failed to load model");

    let mut params = FullParams::new(SamplingStrategy::Greedy { n_past: 0 });

    params.set_n_threads(4);
    params.set_language("es");
    params.set_translate(false);
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    params.set_print_realtime(false);

    //println!("Say something...");
    //let rec = audio_recording::playback();
    //let mut rec = block_on(rec);
    //println!("Recorded!");


    /*let mut pcm: Vec<u8> = Vec::new();
    for frame in rec.buffer.iter() {
        let sample: f32 = frame.channels()[0].into();
        pcm.extend(sample.to_le_bytes().iter());
    }
    std::fs::write("pcm.raw", pcm.as_slice()).expect("Failed to write file");

    let audio_data = rec.buffer.as_f32_slice();
*/

    //let audio_data = whisper_rs::convert_stereo_to_mono_audio(&audio_data);

    // assume we have a buffer of audio data
    // here we'll make a fake one, integer samples, 16 bit, 16KHz, stereo
    /*let mut reader = hound::WavReader::open("cosa2.wav").unwrap();
    let audio_data : Result<Vec<i16>, hound::Error> = reader.samples::<i16>().collect();
    let mut audio_data = audio_data.unwrap();*/


    //let audio_data = whisper_rs::convert_integer_to_float_audio(&audio_data);

    //let audio_data = data.as_thirty_two_float().unwrap();

    // we must convert to 16KHz mono f32 samples for the model
    // some utilities exist for this
    // note that you don't need to use these, you can do it yourself or any other way you want
    // these are just provided for convenience
    // SIMD variants of these functions are also available, but only on nightly Rust: see the docs
    /*let audio_data = whisper_rs::convert_stereo_to_mono_audio(
        &whisper_rs::convert_integer_to_float_audio(&audio_data),
    );*/


    let audio_data = whisper_rs::convert_integer_to_float_audio(&audio_data);

    // now we can run the model
    ctx.full(params, &audio_data[..])
        .expect("failed to run model");

    // fetch the results
    let num_segments = ctx.full_n_segments();
    for i in 0..num_segments {
        let segment = ctx.full_get_segment_text(i).expect("failed to get segment");
        let start_timestamp = ctx.full_get_segment_t0(i);
        let end_timestamp = ctx.full_get_segment_t1(i);
        println!("[{} - {}]: {}", start_timestamp, end_timestamp, segment);
    }
}
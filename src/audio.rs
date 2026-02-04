use rodio;

pub fn play_notification_sound(sound_path: &str) {
    let stream_handle = match rodio::OutputStreamBuilder::open_default_stream(){
        Ok(stream_handle) => stream_handle,
        Err(error) => {
            eprint!("Could not open Output Stream");
            eprint!("\t {:?}", error);
            return;
        } 
    };

    let sink = rodio::Sink::connect_new(stream_handle.mixer());

    let file = match std::fs::File::open(sound_path){
        Ok(file) => file,
        Err(error) => {
            eprint!("Could not open audio file");
            eprint!("\t {:?}", error);
            return;
        }
    };
    let source = match rodio::Decoder::try_from(file) {
        Ok(source) => source,
        Err(error) => {
            eprint!("Could not play audio file");
            eprint!("\t {:?}", error);
            return;
        }
    };
    sink.append(source);
    sink.sleep_until_end();
}
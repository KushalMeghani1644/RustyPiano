use rodio::{source::SineWave, OutputStream, Sink, Source};
use std::io::{stdin, stdout, Write};

fn frequency_from_key(key: char) -> f32 {
    match key {
        'a' => 240.0, // A4
        's' => 270.0, // B4
        'd' => 300.0, // C5
        'f' => 320.0, // D5
        'g' => 360.0, // E5
        'h' => 400.0, // F5
        'j' => 450.0, // G5
        'k' => 480.0, //H6
        _ => 0.0,
    }
}

fn main() {
    // Setup audio output
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    println!("RustyPiano - Press keys a,s,d,f,g,h,j,k to play notes, q to quit");

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let key = input.chars().next().unwrap_or('\n');

        if key == 'q' {
            break;
        }

        let freq = frequency_from_key(key);
        if freq == 0.0 {
            continue;
        }

        // Create a sink for this note
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Sine wave for 1 second
        let source = SineWave::new(freq)
            .amplify(0.20)
            .take_duration(std::time::Duration::from_millis(500));
        sink.append(source);

        // Detach sink to play asynchronously
        sink.detach();
    }
}

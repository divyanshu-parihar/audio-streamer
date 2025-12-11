// module declarations
mod models;
use log::{error, info};
use models::environment::Environment;
use rodio::Decoder;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        error!("Usage: cargo run <file_path> <server|client>");
        return;
    }
    let file_name = String::from(&args[1]);
    let is_server = args[2] == "server";
    let env = Environment::new(&file_name, is_server);

    if is_server {
        let listener = match TcpListener::bind("127.0.0.1:6767") {
            Ok(listener) => listener,
            Err(e) => {
                error!("Error creating a listener {}", e);
                return;
            }
        };

        let mut count = 1;
        info!("Server listening on 127.0.0.1:6767");
        match stream {
                Ok(_stream) => {
                    info!("Stream client connected {count}");

                    // Clone the filename string to move into the thread
                    let sound_file_path = env.file_name.to_string();

                    // Spawn a thread to play audio so we don't block the accept loop
                    thread::spawn(move || {
                        // Use the API pattern that was previously working/detected
                        let stream_handle = match rodio::OutputStreamBuilder::open_default_stream()
                        {
                            Ok(handle) => handle,
                            Err(e) => {
                                error!("Failed to open default audio stream: {}", e);
                                return;
                            }
                        };

                        let sink = rodio::Sink::connect_new(&stream_handle.mixer());

                        match File::open(&sound_file_path) {
                            Ok(file) => {
                                let reader = BufReader::new(file);
                                match Decoder::new(reader) {
                                    Ok(source) => {
                                        sink.append(source);
                                        sink.sleep_until_end();
                                    }
                                    Err(e) => error!("Error decoding audio file: {}", e),
                                }
                            }
                            Err(e) => error!("Error opening file {}: {}", sound_file_path, e),
                        }
                    });

                    count += 1;
                }
                Err(e) => {
                    error!("Couldn't connect client to server: {}", e)
                }
            }
        }
    } else {
        let stream = TcpStream::connect("127.0.0.1:6767");

        match stream {
            Ok(_) => info!("Connected to server"),
            Err(e) => {
                error!("Couldn't connect to server: {}", e)
            }
        }
    }
    // Keep main thread alive if needed, though the loop handles it for server
    if !is_server {
        thread::sleep(Duration::from_secs(5));
    }
}

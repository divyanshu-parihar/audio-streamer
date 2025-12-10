// module declarations
mod models;
use log::{error, info};
use models::environment::Environment;
use rodio::{source::Source, OutputStreamBuilder, Sink};
use std::env;
use std::fs::File;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let file_name = String::from(&args[1]);
    let is_server = if (&args[2] == &String::from("server")) {
        true
    } else {
        false
    };
    let env = Environment::new(&file_name, is_server);
    let mut f = File::open(env.file_name).unwrap();
    //let metadata = f.metadata().unwrap();
    //info!("file opened");
    // info!("File size: {} bytes", metadata.len());
    let mut buffer = [0; 8];
    let data = f.read(&mut buffer);
    match data {
        Ok(size) => info!("Read {:?} bytes from the file", buffer),
        Err(e) => error!("Error reading file: {}", e),
    }
    if (is_server) {
        let outputStream = OutputStreamBuilder::open_default_stream().unwrap();
        let sink = rodio::play(&outputStream.mixer(), f);
        let listener = match TcpListener::bind("127.0.0.1:6767") {
            Ok(listener) => listener,
            Err(e) => {
                error!("Error creating a listener {}", e);
                return;
            }
        };
        let mut count = 1;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    info!("Stream client connected {count}");
                    count += 1;
                }
                Err(e) => {
                    error!("couldn't connect client to server")
                }
            }
        }
    } else {
        let stream = TcpStream::connect("127.0.0.1:6767");

        match stream {
            Ok(_) => info!("Connected to server"),
            Err(e) => {
                error!("couldn't connect to server: {}", e)
            }
        }
    }
    thread::sleep(Duration::from_secs(5));
}


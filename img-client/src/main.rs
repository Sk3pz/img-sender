use std::{net::TcpStream, env};
use std::io::Cursor;
use image::io::Reader as ImageReader;

use chrono::Local;
use img_common::{error_with_err, info, error, highlighted_info};
use send_it::writer::VarWriter;

fn main() {
    // get the file from arguments
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    
    if args.is_empty() {
        error("Must specify a file!");
        return;
    }
    // get test_image.png
    let filename = args.join(" ");
    println!("Filename: {}", filename);
    let img = ImageReader::open(&filename).unwrap().decode().unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png).expect("Failed to write image file!");

    let timestamp = Local::now();
    
    info("Attempting connection...");
    let connection_attempt = TcpStream::connect("localhost:3333");
    if let Err(e) = connection_attempt {
        error_with_err("Failed to connect to server", e);
        return;
    }
    
    let mut connection = connection_attempt.unwrap();
    highlighted_info("Successfully connected to", "localhost:3333");
    
    // create the writer
    let mut writer = VarWriter::default();
    
    writer.add_string(filename);
    writer.add_string(format!("{}", timestamp.format("%Y-%m-%d %H:%M:%S")));
    writer.add_raw(&bytes[..]);
    
    writer.send(&mut connection).expect("Failed to send data!");
    
    error("Connection closed.");
}

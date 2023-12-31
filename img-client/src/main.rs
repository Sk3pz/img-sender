use std::path::Path;
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
    // get the image
    let raw_path = args.join(" ");
    let path = Path::new(&raw_path);
    let filename = path.file_name().expect("Failed to get file name!");
    println!("Filename: {}", filename.to_string_lossy());
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png).expect("Failed to write image file!");

    // get the timestamp
    let timestamp = Local::now();
    
    // connect to the server
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
    
    // create the segments
    writer.add_string(filename.to_string_lossy());
    writer.add_string(format!("{}", timestamp.format("%Y-%m-%d %H:%M:%S")));
    writer.add_raw(&bytes[..]);
    
    // send the segments to the server
    writer.send(&mut connection).expect("Failed to send data!");
    
    // close the client
    error("Connection closed.");
}

use std::{net::TcpListener, thread};

use std::io::Cursor;
use image::io::Reader as ImageReader;

use img_common::{info, highlighted_info, error_with_err, error, conn_log};
use send_it::{reader::VarReader};

fn main() {
    // simple tcp server:
    info("Starting TCP listener...");
    let listener = TcpListener::bind("0.0.0.0:3333").expect("Failed to start tcp listener!");
    highlighted_info("Listnening on port", "3333");
    
    // loop through incoming connections
    for stream in listener.incoming() {
        if let Err(e) = stream {
            error_with_err("Error accepting incoming stram", e);
            continue;
        }
        // get the stream
        let mut stream = stream.unwrap();
        
        // spawn a new thread for the connection
        thread::spawn(move|| {
            // get the ip
            let ip_result = stream.peer_addr();
            if let Err(e) = ip_result {
                error_with_err("Error getting client ip address", e);
                return;
            }
            let ip = ip_result.unwrap();
            
            // create the reader to wrap the stream
            let mut reader = VarReader::new(&mut stream);
            
            // read data from the stream
            while let Ok(read) = reader.read_data() {

                // get each expected segment
                let name_full = read.first().unwrap().to_string();
                let name = name_full.replace(".png", "");
                let timestamp = read.get(1).unwrap();
                let content = read.last().unwrap();

                // print info about the recieved data
                conn_log(ip.to_string(), format!("File name {} taken at {} recieved!", name, timestamp));

                // make the file and write the content to it
                let path = format!("{}_copy.png", name);
                let img = ImageReader::new(Cursor::new(content.as_ref()))
                .with_guessed_format().expect("Failed to write image").decode().expect("Failed to decode!");

                // save the image copy
                img.save(&path).expect("Failed to save new file!");
            }
            
            conn_log(ip.to_string(), "Connection closed.");
        });
    }
    error("Closing server!");
}

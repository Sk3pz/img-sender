use std::{net::TcpListener, thread};

use img_common::{info, highlighted_info, error_with_err, error, conn_log};
use send_it::{reader::VarReader, Segment};

fn main() {
    // simple tcp server:
    info("Starting TCP listener...");
    let listener = TcpListener::bind("0.0.0.0:3333").expect("Failed to start tcp listener!");
    highlighted_info("Listnening on port", "3333");
    
    for stream in listener.incoming() {
        if let Err(e) = stream {
            error_with_err("Error accepting incoming stram", e);
            continue;
        }
        let mut stream = stream.unwrap();
        
        thread::spawn(move|| {
            // handle connections
            // get the ip
            let ip_result = stream.peer_addr();
            if let Err(e) = ip_result {
                error_with_err("Error getting client ip address", e);
                return;
            }
            let ip = ip_result.unwrap();
            
            let mut reader = VarReader::new(&mut stream);
            
            while let Ok(read) = reader.read_data() {
                conn_log(ip.to_string(), format!("Read from client: {:?}", Segment::to_readable(read)));
            }
            
            conn_log(ip.to_string(), "Connection closed.");
        });
    }
    error("Closing server!");
}

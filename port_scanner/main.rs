use std::io;
use std::net::TcpStream;

fn main() {
    let mut ip_address = String::new();
    let start_port = 1; // port to start at
    let end_port = 65535; // port to end at
    let mut working_ports = Vec::new(); // vector with the working ports

    println!("Please enter a IP Address:");
    io::stdin().read_line(&mut ip_address).expect("Failed to read line"); // get user input of which ip address or website to scan
    let mut cur_port = start_port; // set cur_port to the starting port
    for n in start_port..end_port { // loop thru the start to end port, testing to see if you can connect to them
    let stream = TcpStream::connect((ip_address.trim(), cur_port)); // start connection
    if let Ok(stream) = stream {
        println!("port success: {}", cur_port);
        working_ports.push(cur_port);
    } else {
        println!("port failure: {}", cur_port);
    }
    // if true, port works
    // if false, port does not work
    cur_port = cur_port + 1; // add one to the current port
    }

    for x in &working_ports {
        println!("succesful port: {}", x);
    } // list the working port
}

// you could add threading to this to make it much faster - or decrease how long it waits for a response

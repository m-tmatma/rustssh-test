use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <username> <password>", args[0]);
        return;
    }

    println!("args[0] : {}", args[0]);
    println!("args[1] : {}", args[1]);
    println!("args[2] : {}", args[2]);

    // Connect to the local SSH server
    let tcp = TcpStream::connect("127.0.0.1:22").unwrap();
    let mut session = ssh2::Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    session.userauth_password(&args[1], &args[2]).unwrap();
    assert!(session.authenticated());

    let mut channel = session.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    let _ = channel.wait_close();
    println!("{}", channel.exit_status().unwrap());
}


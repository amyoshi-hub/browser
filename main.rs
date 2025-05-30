use std::env;
use std::io::{self, Write, Read};
use std::net::{TcpStream, UdpSocket};

const SERVER_IP: &str = "127.0.0.1";

fn client(port: u16, use_udp: bool) -> io::Result<String> {
    let request = b"GET /index.html HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    if use_udp {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        let server_addr = format!("{}:{}", SERVER_IP, port);
        socket.send_to(request, &server_addr)?;
        
        let mut buf = [0u8; 4096];
        let (amt, _) = socket.recv_from(&mut buf)?;
        Ok(String::from_utf8_lossy(&buf[..amt]).to_string())
    } else {
        let server_addr = format!("{}:{}", SERVER_IP, port);
        let mut stream = TcpStream::connect(server_addr)?;
        stream.write_all(request)?;
        
        let mut response = String::new();
        stream.read_to_string(&mut response)?;
        Ok(response)
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <port> <protocol (tcp/udp)>", args[0]);
        std::process::exit(1);
    }
    let port: u16 = args[1].parse().expect("Invalid port number");
    let protocol = &args[2];
    let use_udp = match protocol.as_str() {
        "udp" => true,
        "tcp" => false,
        _ => {
            eprintln!("Invalid protocol. Use 'tcp' or 'udp'.");
            std::process::exit(1);
        }
    };

    println!("Using {} protocol:", if use_udp { "UDP" } else { "TCP" });
    let response = client(port, use_udp)?;
    println!("{}", response);

    Ok(())
}

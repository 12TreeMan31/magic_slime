use crate::requests::*;
use crate::sockets::*;
use std::collections::HashMap;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str;

mod requests;
mod sockets;

fn holepunch(a: &SocketAddr) {
    let mut peer: UdpWrapper = UdpWrapper::new();
    peer.bind(a);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please specify a port!");
        return;
    }

    // Creates local socket
    let local: SocketAddr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        args[1].parse().unwrap(),
    );
    let mut sfd: UdpWrapper = UdpWrapper::new();
    sfd.bind(&local);
    println!("Starting server on {:?}", local);

    // Temp
    example_calls();

    let mut active_users: HashMap<SocketAddr, UdpWrapper> = HashMap::new();

    'running: loop {
        let mut buf: [u8; 1024] = [0; 1024];
        // Get data
        let (bytes, addr): (usize, SocketAddr) = match sfd.recv_from(&mut buf) {
            Ok((bytes, addr)) => (bytes, addr),
            Err(e) => {
                println!("Failed to get a message: {e}");
                continue 'running;
            }
        };
        let req: Request = match Request::validate(&buf[..bytes]) {
            Some(req) => {
                println!("SYSTEM: {}", req.to_string());
                // TODO
                sfd.send_to("Ok!".to_string().as_bytes(), addr).unwrap();
                req
            }
            None => {
                println!(
                    "{:?}: {}",
                    addr,
                    str::from_utf8(&buf[..bytes]).unwrap().trim()
                );
                continue 'running;
            }
        };
        // New user
        match active_users.insert(addr, UdpWrapper::new()) {
            Some(_) => (),
            None => println!("SYSTEM: New user {:?}", addr),
        }

        match req {
            Request::CONNECT { username, ip, port } => {
                let user_info: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::from(ip)), port);
                let mut peer: UdpWrapper = UdpWrapper::new();
                peer.bind(&user_info);
                peer.send_to(format!("Hello Person").as_bytes(), user_info)
                    .unwrap();
                active_users.insert(user_info, peer);

                println!("SYSTEM: New user {:?}", user_info);
            }
            Request::POST { to, message } => {}
            Request::Error => (),
        }
    }
}

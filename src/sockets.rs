use std::net::{SocketAddr, UdpSocket};
use std::ops::Deref;
use std::os::fd::{AsRawFd, FromRawFd};

#[derive(Debug, Clone, Copy)]
struct Endpoint {
    ip: [u8; 4],
    port: u16,
}

impl Endpoint {
    // Simple wrapper function to interface with the c api
    fn new(addr: &SocketAddr) -> Self {
        let (ip, port): ([u8; 4], u16) = match *addr {
            SocketAddr::V4(addr) => (addr.ip().octets(), addr.port()),
            SocketAddr::V6(_) => ([0, 0, 0, 0], 0),
        };
        Endpoint { ip, port }
    }
}

#[derive(Debug)]
pub struct UdpWrapper(UdpSocket);

extern "C" {
    fn socket_bind(fd: i32, ip: &[u8; 4], port: u16) -> i32;
    fn socket_create() -> i32;
}

impl Deref for UdpWrapper {
    type Target = UdpSocket;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UdpWrapper {
    /// Create an empty socket that can be futher manipulated
    pub fn new() -> Self {
        let fd: i32 = unsafe { socket_create() };
        if fd < 0 {}
        UdpWrapper(unsafe { UdpSocket::from_raw_fd(fd) })
    }
    /// Binds to 'a'
    pub fn bind(&mut self, a: &SocketAddr) {
        let a: Endpoint = Endpoint::new(a);
        let fd: i32 = self.0.as_raw_fd();
        unsafe { socket_bind(fd, &a.ip, a.port) };
    }
}

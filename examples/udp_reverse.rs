#[cfg(feature = "std")]
use std::net::UdpSocket;
#[cfg(not(feature = "std"))]
use w13e_wasi_socket::UdpSocket;

fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or(0.to_string());
    let socket = UdpSocket::bind(format!("127.0.0.1:{}", port))?;

    let mut buf = [0; 128];
    let (size, addr) = socket.recv_from(&mut buf)?;

    let buf = &mut buf[..size];
    buf.reverse();

    socket.send_to(buf, &addr)?;
    Ok(())
}

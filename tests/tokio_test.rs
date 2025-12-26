#![cfg(feature = "tokio")]

use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use socket_pktinfo::AsyncPktInfoUdpSocket;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::test]
async fn async_ipv4_basic_test() -> io::Result<()> {
    let port = 19000;
    let local_ip = Ipv4Addr::LOCALHOST;
    let local_addr: SockAddr = SocketAddr::new(IpAddr::V4(local_ip), port).into();

    let mut buf = [0; 1024];
    let socket =
        AsyncPktInfoUdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port))
            .await?;

    // Send a test packet
    {
        let output_socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
        let data = "Hello Async";
        output_socket.send_to(data.as_bytes(), &local_addr)?;
    }

    let (bytes_received, info) = socket.recv(&mut buf).await?;
    assert!(bytes_received > 0);
    assert_eq!(info.addr_dst, IpAddr::V4(local_ip));
    println!(
        "Async: {} bytes received on interface index {} from src {} with destination ip {}",
        bytes_received, info.if_index, info.addr_src, info.addr_dst,
    );

    Ok(())
}

#[tokio::test]
async fn async_from_std_test() -> io::Result<()> {
    let port = 19001;
    let local_ip = Ipv4Addr::LOCALHOST;
    let local_addr: SockAddr = SocketAddr::new(IpAddr::V4(local_ip), port).into();

    // Create a standard UDP socket and convert to AsyncPktInfoUdpSocket
    let std_socket =
        std::net::UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port))?;
    let socket = AsyncPktInfoUdpSocket::from_std(std_socket)?;

    let mut buf = [0; 1024];

    // Send a test packet
    {
        let output_socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
        let data = "Hello from_std";
        output_socket.send_to(data.as_bytes(), &local_addr)?;
    }

    let (bytes_received, info) = socket.recv(&mut buf).await?;
    assert!(bytes_received > 0);
    assert_eq!(info.addr_dst, IpAddr::V4(local_ip));
    println!(
        "from_std: {} bytes received on interface index {} from src {} with destination ip {}",
        bytes_received, info.if_index, info.addr_src, info.addr_dst,
    );

    Ok(())
}

#[tokio::test]
async fn async_send_to_test() -> io::Result<()> {
    let send_port = 19002;
    let recv_port = 19003;
    let local_ip = Ipv4Addr::LOCALHOST;

    let send_socket = AsyncPktInfoUdpSocket::bind(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::UNSPECIFIED),
        send_port,
    ))
    .await?;

    let recv_socket = AsyncPktInfoUdpSocket::bind(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::UNSPECIFIED),
        recv_port,
    ))
    .await?;

    let target_addr = SocketAddr::new(IpAddr::V4(local_ip), recv_port);
    let data = b"Test send_to";

    let bytes_sent = send_socket.send_to(data, &target_addr).await?;
    assert_eq!(bytes_sent, data.len());

    let mut buf = [0; 1024];
    let (bytes_received, info) = recv_socket.recv(&mut buf).await?;
    assert_eq!(bytes_received, data.len());
    assert_eq!(&buf[..bytes_received], data);
    assert_eq!(info.addr_dst, IpAddr::V4(local_ip));

    println!(
        "send_to: {} bytes sent and received successfully",
        bytes_received
    );

    Ok(())
}

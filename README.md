# socket-pktinfo

[![Build](https://github.com/pixsper/socket-pktinfo/actions/workflows/build.yml/badge.svg)](https://github.com/pixsper/socket-pktinfo/actions)
[![Cargo](https://img.shields.io/crates/v/socket-pktinfo.svg)](https://crates.io/crates/socket-pktinfo/)
[![docs.rs](https://img.shields.io/docsrs/socket-pktinfo)](https://docs.rs/socket-pktinfo/latest/socket-pktinfo/)
[![Rust version: 1.71+](https://img.shields.io/badge/rust%20version-1.71+-orange)](https://blog.rust-lang.org/2023/07/13/Rust-1.71.0/)

Small library to allow cross-platform handling of IP_PKTINFO and IPV6_PKTINFO with socket2 crate. Primary use case for this crate is to determine if a UDP packet was sent to a unicast, broadcast or multicast IP address. Compatible with Windows, Linux and macOS.

> [!IMPORTANT]
> This fork adds asynchronous Tokio support and is implemented entirely by AI without active human review. Use at your own risk.

## Features

- **Synchronous API**: `PktInfoUdpSocket` for blocking I/O operations
- **Asynchronous API** (optional): `AsyncPktInfoUdpSocket` for async/await with Tokio runtime
- **Multi-platform support**: Works on Windows (IOCP), Linux (epoll), and macOS (kqueue)
- **Standard library interop**: Convert from/to `std::net::UdpSocket`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
socket-pktinfo = "0.4"

# For Tokio async support
socket-pktinfo = { git = "https://github.com/minetake01/socket-pktinfo.git", branch = "tokio-support", features = ["tokio"] }
```

## Examples

### Synchronous API

```rust
use std::net::{Ipv4Addr, SocketAddrV4};
use socket2::{Domain, SockAddr};
use socket_pktinfo::PktInfoUdpSocket;

fn main() -> std::io::Result<()> {

    let mut buf = [0; 1024];
    let mut socket = PktInfoUdpSocket::new(Domain::IPV4)?;
    socket.bind(&SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000).into())?;
        
    match socket.recv(&mut buf) {//!
         Ok((bytes_received, info)) => {
             println!("{} bytes received on interface index {} from src {} with destination ip {}",
              bytes_received, info.if_index, info.addr_src, info.addr_dst);
         }
         Err(e) => {
             eprintln!("Error receiving packet - {}", e);
         }
    }
     
    Ok(())
}
```

### Asynchronous API with Tokio

```rust
use std::net::{Ipv4Addr, SocketAddrV4};
use socket2::{Domain, SockAddr};
use socket_pktinfo::AsyncPktInfoUdpSocket;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut buf = [0; 1024];
    
    // Bind directly using the async constructor
    let socket = AsyncPktInfoUdpSocket::bind(
        Domain::IPV4,
        &SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000).into()
    ).await?;
    
    match socket.recv(&mut buf).await {
        Ok((bytes_received, info)) => {
            println!("{} bytes received on interface index {} from src {} with destination ip {}",
                bytes_received, info.if_index, info.addr_src, info.addr_dst);
        }
        Err(e) => {
            eprintln!("Error receiving packet - {}", e);
        }
    }
    
    Ok(())
}
```

### Convert from std::net::UdpSocket

```rust
use socket_pktinfo::AsyncPktInfoUdpSocket;
use std::net::{Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Create a standard UDP socket
    let std_socket = std::net::UdpSocket::bind(
        SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 8000)
    )?;
    
    // Convert to AsyncPktInfoUdpSocket
    let socket = AsyncPktInfoUdpSocket::from_std(std_socket)?;
    
    let mut buf = [0; 1024];
    let (bytes, info) = socket.recv(&mut buf).await?;
    println!("Received {} bytes with pktinfo: {:?}", bytes, info);
    
    Ok(())
}
```

## Platform-Specific Implementation

- **Windows**: Uses `WSARecvMsg` with IOCP for efficient async I/O
- **Unix/Linux**: Uses `recvmsg` with control messages
- **macOS**: Uses `recvmsg` with BSD-style control messages

## Disclaimer

This fork extends `socket-pktinfo` with asynchronous support. The implementation was created fully by AI and has not been actively audited or reviewed by humans. While automated tests pass on supported platforms, there may be edge cases or platform-specific behaviors that are unaddressed. You should evaluate and test in your environment before relying on it in production. Use at your own risk.

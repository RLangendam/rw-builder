use rw_builder::{RwBuilder, TcpStreamBuilder};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

#[test]
fn test_tcp_stream_builder() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind TcpListener");
    let addr = listener.local_addr().expect("Failed to get local addr");

    let server_handle = thread::spawn(move || {
        // First connection (writer)
        let (mut socket1, _) = listener.accept().expect("Failed to accept writer connection");
        let mut buf = [0u8; 12];
        socket1.read_exact(&mut buf).expect("Failed to read from client writer");
        assert_eq!(&buf, b"Hello Server");

        // Second connection (reader)
        let (mut socket2, _) = listener.accept().expect("Failed to accept reader connection");
        socket2.write_all(b"Hello Client").expect("Failed to write to client reader");
    });

    let builder = TcpStreamBuilder::new(addr);
    assert!(format!("{:?}", builder).contains("Builder"));

    let mut writer = builder.writer().expect("Failed to create writer");
    writer.write_all(b"Hello Server").expect("Failed to send data");

    let mut reader = builder.reader().expect("Failed to create reader");
    let mut response = [0u8; 12];
    reader.read_exact(&mut response).expect("Failed to read response");
    assert_eq!(&response, b"Hello Client");

    server_handle.join().expect("Server thread panicked");
}

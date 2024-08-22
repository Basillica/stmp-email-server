use std::net::TcpStream;
use std::io::{Read, Write};
use std::time::Duration;

pub struct ConnectionHandler {
    stream: TcpStream,
    timeout: Duration,
}

impl ConnectionHandler {
    pub fn new(stream: TcpStream, timeout: Duration) -> Self {
        ConnectionHandler { stream, timeout }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<(), std::io::Error> {
        // Read data from the client socket with a timeout
        // ...
        Ok(())
    }

    // usize
    pub fn write(&mut self, buf: &[u8]) -> Result<(), std::io::Error> {
        // Write data to the client socket
        // ...
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), std::io::Error> {
        // Gracefully close the client connection
        // ...
        Ok(())
    }
}
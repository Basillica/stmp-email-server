mod smtp_server;
mod connection_handler;
mod command_parser;
mod message_delivery;
mod authentication;
mod encryption;
mod error_handling;


use std::net::SocketAddr;
use std::time::Duration;

use smtp_server::SmtpServer;

use crate::authentication::{AuthMechanism, Authentication};
use crate::encryption::{Encryption, EncryptionProtocol};
use crate::message_delivery::MessageDelivery;


fn main() {
    // Configure server settings
    let server_addr: SocketAddr = "0.0.0.0:25".parse().unwrap();
    let max_connections = 1024;
    let timeout = Duration::from_secs(60);

    // Initialize components
    let relay_host: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let message_delivery = MessageDelivery::new(relay_host);

    let auth_mechanisms = vec![AuthMechanism::Plain, AuthMechanism::CramMd5];
    let authentication = Authentication::new(auth_mechanisms);

    // let tls_config = Arc::new(rustls::ServerConfig::new(/* Configure TLS settings */));
    let encryption_protocols = vec![EncryptionProtocol::Tls];
    let encryption = Encryption::new(encryption_protocols);

    // Start the SMTP server
    let server = SmtpServer::new(
        server_addr,
        max_connections,
        timeout,
        message_delivery,
        authentication,
        encryption,
    );
    server.run().unwrap();
}
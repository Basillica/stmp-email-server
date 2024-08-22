use std::net::TcpStream;
use std::io::{BufReader, BufWriter, BufRead, Write};


use crate::authentication::{AuthError, AuthMechanism, Authentication};
use crate::encryption::{Encryption, EncryptionError, EncryptionProtocol};
use crate::message_delivery::{DeliveryError, MessageDelivery};
use crate::smtp_server::ClientState;


pub enum SmtpCommand<'a> {
    Helo(String),
    EhloHelo(String),
    Mail(String),
    Rcpt(String),
    Data,
    Rset,
    Noop,
    Quit,
    Unknown(String),
    From(String),
    Auth(AuthMechanism, &'a [u8]),
}

#[derive(Debug)]
pub enum ParseError {
    BadError,
    InvalidCommand
}

pub fn parse_command(input: &str) -> Result<SmtpCommand, ParseError> {
    let mut parts = input.trim().split_whitespace();
    println!("the frigging parts: {:?}", parts);
    match parts.next().unwrap_or("") {
        "HELO" => {
            let domain = parts.next().ok_or(ParseError::InvalidCommand)?;
            Ok(SmtpCommand::Helo(domain.to_string()))
        }
        "FROM" => {
            let domain = parts.next().ok_or(ParseError::InvalidCommand)?;
            Ok(SmtpCommand::From(domain.to_string()))
        }
        "EHLO" => {
            let domain = parts.next().ok_or(ParseError::InvalidCommand)?;
            Ok(SmtpCommand::EhloHelo(domain.to_string()))
        }
        "MAIL" => {
            let from = parts.skip(1).next().ok_or(ParseError::InvalidCommand)?;
            Ok(SmtpCommand::Mail(from.to_string()))
        }
        "RCPT" => {
            let to = parts.skip(1).next().ok_or(ParseError::InvalidCommand)?;
            Ok(SmtpCommand::Rcpt(to.to_string()))
        }
        "DATA" => Ok(SmtpCommand::Data),
        "RSET" => Ok(SmtpCommand::Rset),
        "NOOP" => Ok(SmtpCommand::Noop),
        "QUIT" => Ok(SmtpCommand::Quit),
        _ => {
            println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>><<<<<");
            Ok(SmtpCommand::Unknown(input.to_string()))
        },
    }
}


pub fn handle_client(
    mut stream: TcpStream,
    message_delivery: MessageDelivery,
    authentication: Authentication,
    encryption: Encryption,
) {
    // Perform initial greeting and negotiation
    {
        let mut writer = BufWriter::new(&stream);
        writer.write_all(b"220 example.com SMTP server ready\r\n").unwrap();
        writer.flush().unwrap();
    }

    // Handle client commands
    let mut state = ClientState::new();
    loop {
        let mut input = String::new();
        {
            let mut reader = BufReader::new(&stream);
            reader.read_line(&mut input).unwrap();
        }

        let command = parse_command(&input).unwrap();
        match command {
            SmtpCommand::Helo(domain) => {
                // Handle HELO command
                println!("HELO: {domain}")
                // ...
            }
            SmtpCommand::From(domain) => {
                // Handle HELO command
                println!("FROM: {domain}")
                // ...
            }
            SmtpCommand::EhloHelo(domain) => {
                // Handle EHLO/HELO command
                println!("ehlo: {domain}")
                // ...
            }
            SmtpCommand::Auth(mechanism, credentials) => {
                // Authenticate the client
                authentication.authenticate(&mechanism, &credentials);
                // ...
            }
            SmtpCommand::Mail(from) => {
                // Handle MAIL FROM command
                println!("MAIL")
                // ...
            }
            SmtpCommand::Rcpt(to) => {
                // Handle RCPT TO command
                // ...
                println!("RCPT")
            }
            SmtpCommand::Data => {
                // Handle DATA command
                // ...
                println!("DATA")
            }
            SmtpCommand::Rset => {
                // Reset the client state
                state = ClientState::new();
                // ...
            }
            SmtpCommand::Noop => {
                // No operation
                // ...
                println!("NOOP")
            }
            SmtpCommand::Quit => {
                // Quit the session
                break;
            }
            SmtpCommand::Unknown(command) => {
                // Handle unknown commands
                // ...
            }
        }
    }

    // Clean up and close the connection
    stream.shutdown(std::net::Shutdown::Both).unwrap();
}

fn deliver_message(
    message_delivery: &MessageDelivery,
    recipients: &[String],
    message: &[u8],
) -> Result<(), DeliveryError> {
    // Attempt to deliver the message to the specified recipients
    message_delivery.deliver(recipients, message)
}

fn authenticate(
    authentication: &Authentication,
    mechanism: &AuthMechanism,
    credentials: &[u8],
) -> Result<(), AuthError> {
    // Authenticate the client using the specified mechanism and credentials
    authentication.authenticate(mechanism, credentials)
}

fn negotiate_encryption(
    encryption: &Encryption,
    protocol: &EncryptionProtocol,
    stream: &mut TcpStream,
) -> Result<(), EncryptionError> {
    // Negotiate and establish a secure connection using the specified protocol
    encryption.negotiate(protocol, stream)
}

// HELO example.com
// MAIL FROM:<sender@example.com>
// RCPT TO:<recipient@example.com>
// DATA
// Subject: Test Email
// This is a test email.
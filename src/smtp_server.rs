use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::error::Error;
use std::time::Duration;

use crate::authentication::Authentication;
use crate::command_parser::handle_client;
use crate::connection_handler::ConnectionHandler;
use crate::encryption::Encryption;
use crate::message_delivery::MessageDelivery;


/// Represents the state of an SMTP client session.
pub struct ClientState {
    authenticated: bool,
    sender: Option<String>,
    recipients: Vec<String>,
}

impl ClientState {
    pub fn new() -> Self {
        ClientState {
            authenticated: false,
            sender: None,
            recipients: Vec::new(),
        }
    }
}


struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

/// The main SMTP server struct.
pub struct SmtpServer {
    addr: SocketAddr,
    max_connections: usize,
    timeout: Duration,
    message_delivery: MessageDelivery,
    authentication: Authentication,
    encryption: Encryption,
}

impl SmtpServer {
    pub fn new(
        addr: SocketAddr,
        max_connections: usize,
        timeout: Duration,
        message_delivery: MessageDelivery,
        authentication: Authentication,
        encryption: Encryption,
    ) -> Self {
        SmtpServer {
            addr,
            max_connections,
            timeout,
            message_delivery,
            authentication,
            encryption,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(self.addr)?;
        let pool = ThreadPool::new(self.max_connections);

        for stream in listener.incoming() {
            // let stream = stream?;
            // let handler = ConnectionHandler::new(stream.unwrap(), self.timeout);
            let message_delivery = self.message_delivery.clone();
            let authentication = self.authentication.clone();
            let encryption = self.encryption.clone();

            pool.execute(move || {
                handle_client(stream.unwrap(), message_delivery, authentication, encryption);
            });
        }

        Ok(())
    }
}
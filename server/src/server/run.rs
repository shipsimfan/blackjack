use super::Server;

impl Server {
    /// Run the server
    pub fn run(mut self) -> ! {
        let mut events = Vec::new();
        loop {
            self.epoll.borrow_mut().wait(None, &mut events).unwrap();

            for event in &events {
                if event.id() == u64::MAX {
                    match self.listen_socket.accept() {
                        Ok(()) => println!("Accepted client!"),
                        Err(error) => eprintln!("Error while accepting client - {}", error),
                    }
                }
            }
        }
    }
}

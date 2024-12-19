use super::{client_socket::ClientSocket, Server};

impl Server {
    /// Run the server
    pub fn run(mut self) -> ! {
        let mut events = Vec::new();
        loop {
            self.epoll.borrow_mut().wait(None, &mut events).unwrap();

            for event in &events {
                if event.id() == u64::MAX {
                    self.accept_client();
                    continue;
                }
            }
        }
    }

    fn accept_client(&mut self) {
        let handle = match self.listen_socket.accept() {
            Ok(handle) => handle,
            Err(error) => return eprintln!("Error while accepting client: {}", error),
        };

        let mut target_slot = None;
        for i in 0..self.clients.len() {
            if self.clients[i].is_none() {
                target_slot = Some(i);
                break;
            }
        }

        let id = match target_slot {
            Some(id) => id,
            None => {
                println!("Client connected but server full");
                return;
            }
        };

        match ClientSocket::new(handle, self.epoll.clone(), id as _) {
            Ok(client_socket) => self.clients[id] = Some(client_socket),
            Err(error) => println!("Error while making a client: {}", error),
        }
    }
}

use super::{client_socket::ClientSocket, Server};
use crate::server::ClientWriter;

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

                let client_id = event.id() as usize;
                let client = match &mut self.clients[client_id] {
                    Some(client) => client,
                    None => continue,
                };

                if let Some(message) = client.read() {
                    todo!("Inform lobby of message");
                }
            }
        }
    }

    fn accept_client(&mut self) {
        let handle = match self.listen_socket.accept() {
            Ok(handle) => handle,
            Err(error) => return eprintln!("[ERROR] Error while accepting client: {}", error),
        };

        let mut target_slot = None;
        for i in 0..self.clients.len() {
            if self.clients[i].is_none() {
                target_slot = Some(i);
                break;
            }
        }

        let client_id = match target_slot {
            Some(id) => id,
            None => {
                let client = ClientWriter::new_unregistered(handle);
                return todo!("Inform lobby of server full");
            }
        };

        match ClientSocket::new(
            handle,
            self.epoll.clone(),
            client_id as _,
            self.clients_to_disconnect.clone(),
        ) {
            Ok((client_socket, client)) => {
                self.clients[client_id] = Some(client_socket);
                todo!("Inform lobby of connected player");
            }
            Err(error) => eprintln!("[ERROR] Error while registering client: {}", error),
        }
    }

    fn disconnect_clients(&mut self) {
        while let Some(client_id) = self.clients_to_disconnect.borrow_mut().pop() {
            if self.clients[client_id].is_none() {
                continue;
            }

            self.clients[client_id] = None;
            todo!("Inform lobby of disconnect");
        }
    }
}

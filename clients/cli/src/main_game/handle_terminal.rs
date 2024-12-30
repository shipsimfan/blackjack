use super::view::ViewEvent;
use crate::{Connection, MainGame, TerminalEvent, VirtualTerminal};
use blackjack::messages::{ChatClientMessage, PlayNextRoundClientMessage};

impl MainGame {
    /// Handle a terminal event, returning true if the program should exit
    pub fn handle_terminal(
        &mut self,
        event: TerminalEvent,
        terminal: &mut VirtualTerminal,
        connection: &mut Connection,
    ) -> bool {
        let event = match event {
            TerminalEvent::Resize => {
                self.view.resize(&self.table, self.client_id, terminal);
                return false;
            }
            _ => match self
                .view
                .handle_terminal(event, &self.table, self.client_id, terminal)
            {
                Some(event) => event,
                None => return false,
            },
        };

        match event {
            ViewEvent::Chat(message) => connection.send(ChatClientMessage::new(message)),
            ViewEvent::PlayNextRound => connection.send(PlayNextRoundClientMessage::new(true)),
            ViewEvent::DontPlayNextRound => connection.send(PlayNextRoundClientMessage::new(false)),
        }

        false
    }
}

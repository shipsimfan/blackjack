use crate::{virtual_terminal::TerminalEvent, AppState, Connection, Options, VirtualTerminal};
use argparse::Command;
use blackjack::messages::ServerMessage;
use win32::{WaitForMultipleObjectsEx, DWORD, FALSE, INFINITE, WAIT_FAILED, WAIT_OBJECT_0};

const WAIT_OBJECT_1: DWORD = WAIT_OBJECT_0 + 1;

/// Run the program
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let options = match Options::parse_env()? {
        Some(options) => options,
        None => return Ok(()),
    };

    let mut terminal = VirtualTerminal::new()?;

    terminal.write(format_args!(
        "Connecting to {}:{} . . . \n",
        options.address(),
        options.port()
    ));

    let mut connection = Connection::connect(options.address(), options.port())?;

    let mut game_state = AppState::new(options)?;

    let wait_handles = [connection.read_event(), terminal.input()];

    loop {
        let event = unsafe {
            WaitForMultipleObjectsEx(
                wait_handles.len() as _,
                wait_handles.as_ptr(),
                FALSE,
                INFINITE,
                FALSE,
            )
        };

        match event {
            WAIT_OBJECT_0 => match connection.read()? {
                Some(Some(ServerMessage::Error(error))) => return Err(Box::new(error)),
                Some(Some(message)) => {
                    match game_state.handle_message(message, &mut terminal, &mut connection) {
                        Some(new_game_state) => game_state = new_game_state,
                        None => return Ok(()),
                    }
                }
                Some(None) => {
                    terminal.write("Disconnect by server!\n");
                    return Ok(());
                }
                None => {}
            },
            WAIT_OBJECT_1 => match terminal.read()? {
                TerminalEvent::Ignored => {}
                TerminalEvent::Exit => return Ok(()),
                event => {
                    if game_state.handle_terminal(event, &mut terminal, &mut connection) {
                        return Ok(());
                    }
                }
            },
            WAIT_FAILED => return Err(Box::new(win32::Error::get_last_error())),
            _ => {}
        }
    }
}

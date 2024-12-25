use crate::{virtual_terminal::TerminalEvent, Connection, GameState, Options, VirtualTerminal};
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

    let mut virtual_terminal = VirtualTerminal::new()?;

    virtual_terminal.write(format_args!(
        "Connecting to {}:{} . . . \n",
        options.address(),
        options.port()
    ));

    let mut connection = Connection::connect(options.address(), options.port())?;

    let mut game_state = GameState::new(options);

    let wait_handles = [connection.event(), virtual_terminal.input()];

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
                    game_state.handle_message(message, &mut virtual_terminal);
                }
                Some(None) => {
                    virtual_terminal.write("Disconnect by server!\n");
                    return Ok(());
                }
                None => {}
            },
            WAIT_OBJECT_1 => match virtual_terminal.read()? {
                TerminalEvent::Ignored => {}
                TerminalEvent::Exit => return Ok(()),
                event => {
                    if game_state.handle_terminal(event, &mut virtual_terminal) {
                        return Ok(());
                    }
                }
            },
            WAIT_FAILED => return Err(Box::new(win32::Error::get_last_error())),
            _ => {}
        }
    }
}

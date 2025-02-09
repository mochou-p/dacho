// dacho/core/game/src/data/commands.rs

use winit::window::CursorGrabMode;


#[non_exhaustive]
pub enum Command {
    Exit,

    SetCursorGrab(CursorGrabMode),
    SetCursorPosition((i32, i32)),
    SetCursorVisible(bool)
}

#[derive(Default)]
#[non_exhaustive]
pub struct Commands {
    pub queue: Vec<Command>
}

impl Commands {
    #[inline]
    pub fn push(&mut self, command: Command) {
        self.queue.push(command);
    }

    #[inline]
    pub fn extend<C>(&mut self, commands: C)
    where
        C: IntoIterator<Item = Command>
    {
        self.queue.extend(commands);
    }
}


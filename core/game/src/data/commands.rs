// dacho/core/game/src/data/commands.rs

#[non_exhaustive]
pub enum Command {
    Exit,
    Noop
}

#[derive(Default)]
#[non_exhaustive]
pub struct Commands {
    pub queue: Vec<Command>
}

impl Commands {
    #[inline]
    pub fn submit(&mut self, command: Command) {
        self.queue.push(command);
    }

    #[inline]
    pub fn submit_all<C>(&mut self, commands: C)
    where
        C: IntoIterator<Item = Command>
    {
        self.queue.extend(commands);
    }
}


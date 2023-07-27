use crate::RwBuilder;
use anyhow::{anyhow, Result};
use std::{
    cell::RefCell,
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

/// Type for building readers and writers on top of a process handle.
/// It is itself an `RwBuilder`, but can't be created through one.
/// This is why we call it a source.
#[derive(Debug)]
pub struct Builder {
    /// The command used to spawn the process to attach a reader and/or writer to.
    command: RefCell<Command>,
}

impl Builder {
    /// Create a builder that spawns a process based on the command being passed.
    #[must_use]
    pub fn new(command: Command) -> Self {
        Self {
            command: command.into(),
        }
    }

    /// Spawn a child process based on the command and return the actual builder of the reader and writer.
    /// # Errors
    /// In case spawning the child process fails the reason why is return as an error.
    pub fn spawn(&self) -> Result<ChildBuilder> {
        let child = self
            .command
            .borrow_mut()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        Ok(ChildBuilder {
            child: child.into(),
        })
    }
}

impl RwBuilder for Builder {
    type Reader = ChildStdout;

    fn reader(&self) -> Result<Self::Reader> {
        let mut child = self.command.borrow_mut().stdout(Stdio::piped()).spawn()?;
        child
            .stdout
            .take()
            .ok_or_else(|| anyhow!("no child stdout"))
    }

    type Writer = ChildStdin;

    fn writer(&self) -> Result<Self::Writer> {
        let mut child = self.command.borrow_mut().stdin(Stdio::piped()).spawn()?;
        child.stdin.take().ok_or_else(|| anyhow!("no child stdin"))
    }
}

/// Type for building readers and writers on top of a spawned child process.
/// It is itself an `RwBuilder`, so it can be chained further.
/// It is also a source.
#[derive(Debug)]
pub struct ChildBuilder {
    /// Handle to the child process
    child: RefCell<Child>,
}

impl RwBuilder for ChildBuilder {
    type Reader = ChildStdout;

    fn reader(&self) -> Result<Self::Reader> {
        self.child
            .borrow_mut()
            .stdout
            .take()
            .ok_or_else(|| anyhow!("No child stdout. Did you already build a reader?"))
    }

    type Writer = ChildStdin;

    fn writer(&self) -> Result<Self::Writer> {
        self.child
            .borrow_mut()
            .stdin
            .take()
            .ok_or_else(|| anyhow!("No child stdin. Did you already build a writer?"))
    }
}

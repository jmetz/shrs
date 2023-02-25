use std::ops::{Deref, DerefMut};

use pino_deref::Deref;

#[derive(Debug)]
pub struct Redirect {
    pub n: Option<IONumber>,
    pub file: Filename,
    pub mode: RedirectMode,
}

#[derive(Debug)]
pub enum RedirectMode {
    Read,
    Write,
    ReadAppend,
    WriteAppend,
    ReadDup,
    WriteDup,
    ReadWrite,
}

#[derive(Debug)]
pub struct Assign {
    pub var: Word,
    pub val: Word,
}

/// Seperator character between commands
#[derive(Debug)]
pub enum SeperatorOp {
    /// Ampersand (&)
    Amp,
    /// Semicolon (;)
    Semi,
}

#[derive(Debug)]
pub enum Command {
    /// Basic command
    ///
    /// ```sh
    /// ls -al
    /// ```
    Simple {
        assigns: Vec<Assign>,
        redirects: Vec<Redirect>,
        args: Vec<Word>,
    },

    /// Two commands joined by a pipe
    ///
    /// ```sh
    /// cat .bashrc | wc -l
    /// ```
    Pipeline(Box<Command>, Box<Command>),

    /// Compound command of And
    And(Box<Command>, Box<Command>),

    /// Compound command of Or
    Or(Box<Command>, Box<Command>),

    /// Negate the exit code of command
    Not(Box<Command>),

    /// Asynchronous list of commands
    ///
    /// ```sh
    /// command1 & command2
    /// ```
    /// We do not wait for `command1` to finish executing before executing `command2`
    AsyncList(Box<Command>, Option<Box<Command>>),

    /// Sequential list of commands
    /// ```sh
    /// command1 ; command2
    /// ```
    /// We wait for `command1` to finish executing before executing `command2`
    SeqList(Box<Command>, Option<Box<Command>>),

    /// No op
    None,
}

#[derive(Debug)]
pub struct Word(pub String);

#[derive(Deref, Debug)]
pub struct Filename(pub String);

#[derive(Deref, Debug)]
pub struct IONumber(pub usize);

impl Deref for Word {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Word {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

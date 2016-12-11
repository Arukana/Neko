use ::dynamic::CompositerError;
use ::graphic::GraphicError;
use ::pty_proc::shell::ShellError;
use std::error::Error;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, NekoError>;

/// The enum `NekoError` defines the possible errors
/// from constructor Neko.
#[derive(Debug)]
pub enum NekoError {
    /// The dynamic library interface has occured an error.
    Dynamic(CompositerError),
    /// The graphic interface has occured an error.
    Graphic(GraphicError),
    /// The shell interface has occured an error.
    Shell(ShellError),
//    /// The terminal size is smaller than the size of the Neko
//    Size,
}

impl fmt::Display for NekoError {
    /// The function `fmt` formats the value using
    /// the given formatter.
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl Error for NekoError {
    /// The function `description` returns a short description of
    /// the error.
    fn description(&self) -> &str {
        match *self {
            NekoError::Dynamic(_) => {
                "The dynamic library interface has\
                                        occured an error."
            }
            NekoError::Graphic(_) => {
                "The graphic interface has\
                                        occured an error."
            }
            NekoError::Shell(_) => "The shell interface has occured an error",
//            NekoError::Size => "Terminal too small",
        }
    }

    /// The function `cause` returns the lower-level cause of
    /// this error if any.
    fn cause(&self) -> Option<&Error> {
        match *self {
            NekoError::Dynamic(ref why) => Some(why),
            NekoError::Graphic(ref why) => Some(why),
            NekoError::Shell(ref why) => Some(why),
          //  NekoError::Size => Some(()),
        }
    }
}

impl From<CompositerError> for NekoError {
    fn from(err: CompositerError) -> NekoError {
        NekoError::Dynamic(err)
    }
}

impl From<GraphicError> for NekoError {
    fn from(err: GraphicError) -> NekoError {
        NekoError::Graphic(err)
    }
}

impl From<ShellError> for NekoError {
    fn from(err: ShellError) -> NekoError {
        NekoError::Shell(err)
    }
}

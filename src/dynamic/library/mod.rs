#[macro_use]
mod macros;
pub mod position;
pub mod state;
mod err;

use std::fmt;
use std::mem;
use std::ptr;
use std::os::unix::ffi::OsStrExt;
use std::cmp::{Eq, Ordering};
use std::path::PathBuf;
use std::ffi::CString;

pub use self::err::{LibraryError, Result};
pub use self::state::LibraryState;
pub use self::position::Position;

use ::libc;
use ::pty;

/// The struct `Library` is a table of callback.
pub struct Library {
    /// `start` interface.
    start: Option<extern fn(state: *const LibraryState, save: *const libc::c_void)>,
    /// `idle` interface.
    idle: Option<extern fn(state: *const LibraryState, save: *const libc::c_void)>,
    /// `idle` interface.
    end: Option<extern fn(state: *const LibraryState, save: *const libc::c_void)>,
    /// `save` pointer to share a segment of librairy memory.
    save: *mut libc::c_void,
    /// dynamic library interface.
    handle: *mut libc::c_void,
    /// priority queue.
    index: i64,
    /// Address of the library.
    path: PathBuf,
    /// .
    unmounted: bool,
}

impl Library {
    /// The constructor method `new` returns a interface for a extern library.
    pub fn new(path: PathBuf, index: i64, state: &LibraryState) -> Result<Self> {
        unsafe {
            let handle: *mut libc::c_void = libc::dlopen(
                path.as_os_str().as_bytes().as_ptr() as *const libc::c_char,
                libc::RTLD_LAZY
            );

            if handle.eq(&ptr::null_mut()) {
                Err(LibraryError::BadDyLib(CString::from_raw(libc::dlerror())
                                                   .into_string()
                                                   .unwrap_or_default()))
            } else {
                let lib: Library = Library {
                    start: symbol!(handle, b"start\0".as_ptr() as *const libc::c_char),
                    idle: symbol!(handle, b"idle\0".as_ptr() as *const libc::c_char),
                    end: symbol!(handle, b"end\0".as_ptr() as *const libc::c_char),
                    save: ptr::null_mut(),
                    handle: handle,
                    index: index,
                    path: path,
                    unmounted: false,
                };

                lib.start(state);
                Ok(lib)
            }
        }
    }

    /// The accessor method `as_path_buf` return address of library.
    pub fn as_path_buf(&self) -> &PathBuf {
        &self.path
    }

    /// The accessor method `get_priority` return level's priority of library.
    pub fn get_priority(&self) -> i64 {
        self.index
    }

    pub fn is_unmounted(&self) -> bool {
        self.unmounted
    }

    /// The method `start` call the extern function if defined.
    pub fn start(&self, state: &LibraryState) {
        if let Some(start) = self.start {
            start(state, self.save);
        }
    }

    /// The method `start` call the extern function if defined.
    pub fn end(&self, state: &LibraryState) {
        if let Some(end) = self.end {
            end(state, self.save);
        }
    }
    /// The method `idle` call the extern function if defined.
    pub fn call(&self, state: &LibraryState, event: &pty::ShellState) {
        if let Some(()) = event.is_idle() {
            if let Some(idle) = self.idle {
                idle(state, self.save);
            }
        }
    }
}

/// Trait for equality comparisons which are equivalence relations.
impl Eq for Library {}

/// Trait for equality comparisons which are partial equivalence relations.
impl PartialEq for Library {
    /// This method tests for `self` and `other` values to be equal, and
    /// is used by `eq`.
    fn eq(&self, rhs: &Library) -> bool {
        self.index.eq(&rhs.index)
    }
}

/// Trait for values that can be compared for a sort-order.
impl PartialOrd for Library {
    /// This method returns an `Ordering` between `self` and `other` values
    /// if one exists.
    fn partial_cmp(&self, rhs: &Library) -> Option<Ordering> {
        self.index.partial_cmp(&rhs.index)
    }
}

/// Trait for equality comparisons which are equivalence relations.
impl Ord for Library {
    /// This method returns an `Ordering` between `self` and `other`.
    fn cmp(&self, rhs: &Library) -> Ordering {
        self.index.cmp(&rhs.index)
    }
}

/// Format trait for the `?` character.
impl fmt::Debug for Library {
    /// Formats the value using the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "library({}): start:{} path:({:?})",
               self.index,
               self.start.is_some(),
               self.path)
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            assert_ne!(libc::dlclose(self.handle), -1);
        }
    }
}

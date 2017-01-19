#[macro_use]
mod macros;
pub mod state;
mod err;

use std::fmt;
use std::mem;
use std::ptr;
use std::os::unix::ffi::OsStrExt;
use std::cmp::{Eq, Ordering};
use std::path::PathBuf;
use std::ffi::CString;
use std::ops::Deref;

pub use self::state::LibraryState;
pub use self::err::{LibraryError, Result};

use ::libc;
use ::pty;

/// The struct `Library` is a table of callback.
pub struct Library {
    /// `install` interface.
    install: Option<extern fn(state: *const LibraryState, save: &*const *const libc::c_void)>,
    /// `uninstall` interface.
    uninstall: Option<extern fn(state: *const LibraryState, save: &*const *const libc::c_void)>,
    /// `mount` interface.
    start: Option<extern fn(state: *const LibraryState, save: &*const *const libc::c_void)>,
    /// `unmount` interface.
    end: Option<extern fn(state: *const LibraryState, save: &*const *const libc::c_void)>,
    /// `idle` interface.
    idle: Option<extern fn(state: *const LibraryState, save: &*const *const libc::c_void)>,
    /// `process` interface.
    process: Option<extern fn(state: *const LibraryState, save: &*const *const libc::c_void, name: *const libc::c_uchar, pid: libc::c_int)>,
    /// `command` interface.
    command: Option<extern fn(state: *const LibraryState, save: &*const *const libc::c_void, line: *const libc::c_uchar)>,
    /// `key_unicode_down` interface.
    key_unicode_down: Option<extern fn(state: *const LibraryState, save: &*const *const libc::c_void, code: libc::c_ulonglong)>,
    /// `key_string_down` interface.
    key_string_down: Option<extern fn(state: *const LibraryState, save: &*const *const libc::c_void, text: *const libc::c_uchar)>,
    /// `key_repeat_down` interface.
    key_repeat_down: Option<extern fn(state: *const LibraryState, save: &*const *const libc::c_void, repeat: libc::c_ulong)>,
    /// `key_interval_down` interface.
    key_interval_down: Option<extern fn(state: *const LibraryState, save: &*const *const libc::c_void, interval: libc::c_longlong)>,
    /// 'mouse_down' interface.
    mouse_down: Option<extern fn(state: *const LibraryState, save: &*const *const libc::c_void, code: libc::c_uint, x: libc::c_ushort, y: libc::c_ushort)>,
    /// 'mouse_up' interface.
    mouse_up: Option<extern fn(state: *const LibraryState, save: &*const *const libc::c_void, code: libc::c_uint, x: libc::c_ushort, y: libc::c_ushort)>,
    /// `input` interface.
    input: Option<extern fn(state: *const LibraryState, save: &*const *const libc::c_void, text: *const libc::c_uchar)>,
    /// `output` interface.
    output: Option<extern fn(state: *const LibraryState, save: &*const *const libc::c_void, text: *const libc::c_uchar)>,
    /// `signal` interface.
    signal: Option<extern fn(state: *const LibraryState, save: &*const *const libc::c_void, number: libc::c_int)>,
    /// `resized` interface.
    resized: Option<extern fn(state: *const LibraryState, save: &*const *const libc::c_void, text: *const pty::Winszed)>,
    /// `save` pointer to share a segment of librairy memory.
    save: *const *const libc::c_void,
    /// dynamic library interface.
    handle: *mut libc::c_void,
    /// priority queue.
    index: i64,
    /// Address of the library.
    path: PathBuf,
    /// Inform to unmount the library.
    unmounted: bool,
}

impl Library {
    /// The constructor method `new` returns a interface for a extern library.
    pub fn new(path: PathBuf, index: i64, state: &LibraryState) -> Result<Self> {
        unsafe {
            let mut libname: Vec<libc::c_uchar> = Vec::with_capacity(4096);
            
            libname.extend_from_slice(&path.as_os_str().as_bytes()[..]);
            libname.push(b'\0');
            let handle: *mut libc::c_void = libc::dlopen(
                libname.as_ptr() as *const libc::c_char,
                libc::RTLD_LAZY
            );
            if handle.eq(&ptr::null_mut()) {
                Err(LibraryError::BadDyLib(CString::from_raw(libc::dlerror())
                                                   .into_string()
                                                   .unwrap_or_default()))
            } else {
                let lib: Library = Library {
                    install: symbol!(handle, b"install\0".as_ptr() as *const libc::c_char),
                    uninstall: symbol!(handle, b"uninstall\0".as_ptr() as *const libc::c_char),
                    start: symbol!(handle, b"start\0".as_ptr() as *const libc::c_char),
                    end: symbol!(handle, b"end\0".as_ptr() as *const libc::c_char),
                    idle: symbol!(handle, b"idle\0".as_ptr() as *const libc::c_char),
                    process: symbol!(handle, b"process\0".as_ptr() as *const libc::c_char),
                    command: symbol!(handle, b"command\0".as_ptr() as *const libc::c_char),
                    key_unicode_down: symbol!(handle, b"key_unicode_down\0".as_ptr() as *const libc::c_char),
                    key_string_down: symbol!(handle, b"key_string_down\0".as_ptr() as *const libc::c_char),
                    key_repeat_down: symbol!(handle, b"key_repeat_down\0".as_ptr() as *const libc::c_char),
                    key_interval_down: symbol!(handle, b"key_interval_down\0".as_ptr() as *const libc::c_char),
                    mouse_down: symbol!(handle, b"mouse_down\0".as_ptr() as *const libc::c_char),
                    mouse_up: symbol!(handle, b"mouse_up\0".as_ptr() as *const libc::c_char),
                    input: symbol!(handle, b"input\0".as_ptr() as *const libc::c_char),
                    output: symbol!(handle, b"output\0".as_ptr() as *const libc::c_char),
                    signal: symbol!(handle, b"signal\0".as_ptr() as *const libc::c_char),
                    resized: symbol!(handle, b"resized\0".as_ptr() as *const libc::c_char),
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

    /// The method `install` call the extern function if defined
    /// when the library is installed.
    pub fn install(&self, state: &LibraryState) {
        if let Some(install) = self.install {
            install(state, &self.save);
        }
    }

    /// The method `uninstall` call the extern function if defined
    /// when the library is uninstalled.
    pub fn uninstall(&self, state: &LibraryState) {
        if let Some(uninstall) = self.uninstall {
            uninstall(state, &self.save);
        }
    }

    /// The method `start` call the extern function if defined
    /// when the library is mounted.
    pub fn start(&self, state: &LibraryState) {
        if let Some(start) = self.start {
            start(state, &self.save);
        }
    }

    /// The method `end` call the extern function if defined
    /// when the library is unmounted.
    pub fn end(&self, state: &LibraryState) {
        if let Some(end) = self.end {
            end(state, &self.save);
        }
    }

    /// The method `end` call the extern function if defined.
    pub fn idle(&self, state: &LibraryState) {
        if let Some(idle) = self.idle {
            idle(state, &self.save);
        }
    }

    /// The method `process` call the extern function if defined
    /// when the child current process as been canged.
    pub fn process(&self, state: &LibraryState, taskname: &[libc::c_uchar], pid: libc::c_int) {
        if let Some(process) = self.process {
            process(state, &self.save, taskname.as_ptr(), pid);
        }
    }

    /// The method `command` call the extern function if defined
    /// when a command line is outputed to the terminal.
    pub fn command(&self, state: &LibraryState, line: &[libc::c_uchar]) {
        if let Some(command) = self.command {
            command(state, &self.save, line.as_ptr());
        }
    }

    /// The method `key_unicode_down` call the extern function if defined
    /// when a key is pressed.
    pub fn key_unicode_down(&self, state: &LibraryState, code: libc::c_ulonglong) {
        if let Some(key_unicode_down) = self.key_unicode_down {
            key_unicode_down(state, &self.save, code);
        }
    }

    /// The method `key_string_down` call the extern function if defined
    /// when a text is pasted or pressed.
    pub fn key_string_down(&self, state: &LibraryState, text: &[libc::c_uchar]) {
        if let Some(key_string_down) = self.key_string_down {
            key_string_down(state, &self.save, text.as_ptr());
        }
    }

    /// The method `key_repeat_down` call the extern function if defined
    /// when a key is held. It determines the time before it begins to
    /// output the held character within a given interval.
    pub fn key_repeat_down(&self, state: &LibraryState, repeat: libc::c_ulong) {
        if let Some(key_repeat_down) = self.key_repeat_down {
            key_repeat_down(state, &self.save, repeat);
        }
    }

    /// The method `key_interval_down` call the extern function if defined
    /// when a key is held. It determines the time between two outputs
    /// repetitions of the held character. (Triggered by key_repeat_down())
    pub fn key_interval_down(&self, state: &LibraryState, interval: libc::c_longlong) {
        if let Some(key_interval_down) = self.key_interval_down {
            key_interval_down(state, &self.save, interval);
        }
    }

    /// The method `mouse_down` call the extern function if defined
    /// when the mouse is pressed.
    pub fn mouse_down(&self, state: &LibraryState, code: libc::c_uint, x: libc::c_ushort, y: libc::c_ushort) {
        if let Some(mouse_down) = self.mouse_down {
            mouse_down(state, &self.save, code, x, y);
        }
    }

    /// The method `mouse_up` call the extern function if defined
    /// when the mouse is released.
    pub fn mouse_up(&self, state: &LibraryState, code: libc::c_uint, x: libc::c_ushort, y: libc::c_ushort) {
        if let Some(mouse_up) = self.mouse_up {
            mouse_up(state, &self.save, code, x, y);
        }
    }

    /// The method `input` call the extern function if defined
    /// when something is inputted to the terminal, whatever it is.
    pub fn input(&self, state: &LibraryState, text: &[libc::c_uchar]) {
        if let Some(input) = self.input {
            input(state, &self.save, text.as_ptr());
        }
    }

    /// The method `output` call the extern function if defined.
    /// when something is outputted to the terminal, whatever it is.
    pub fn output(&self, state: &LibraryState, text: &[libc::c_uchar]) {
        if let Some(output) = self.output {
            output(state, &self.save, text.as_ptr());
        }
    }

    /// The method `resized` call the extern function if defined
    /// when the window is resized.
    pub fn resized(&self, state: &LibraryState, size: &pty::Winszed) {
        if let Some(resized) = self.resized {
            resized(state, &self.save, size);
        }
    }

    /// The method `signal` call the extern function if defined.
    /// when a signal is handled
    pub fn signal(&self, state: &LibraryState, number: libc::c_int) {
        if let Some(signal) = self.signal {
            signal(state, &self.save, number);
        }
    }

    /// The method `call` will read the ShellState to call an adapted extern function if defined.
    pub fn call(&self, state: &LibraryState, event: &pty::ShellState) {
        if let Some(()) = event.is_idle() {
            if let Some(idle) = self.idle {
                idle(state, &self.save);
            }
        } else {
            if let Some(num) = event.is_signal() {
                self.signal(state, num);
            } else {
                if let Some(key) = event.is_input_keydown() {
                    match key {
                        pty::Key::Char(code) => self.key_unicode_down(state, code),
                        pty::Key::Str(text) => self.key_string_down(state, &text.deref()),
                    }
                } else if let Some(repeat) = event.is_input_keyrepeat() {
                    self.key_repeat_down(state, repeat)
                } else if let Some(interval) = event.is_input_keyinterval() {
                    self.key_interval_down(state, interval)
                } else if let Some((mouse, pressed, x, y)) = event.is_input_mouse() {
                    if pressed {
                        self.mouse_down(state, mouse as u32, x, y)
                    } else {
                        self.mouse_up(state, mouse as u32, x, y)
                    }
                } else if let Some(slice) = event.is_input_slice() {
                    self.input(state, slice)
                } else if let Some(slice) = event.is_output_last() {
                    self.output(state, slice)
                } else if let Some(&(pid, name)) = event.is_task() {
                    self.process(state, &name[..], pid)
                }
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

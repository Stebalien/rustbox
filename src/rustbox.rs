#![feature(std_misc)]

extern crate libc;
extern crate termbox_sys as termbox;
#[macro_use] extern crate bitflags;

pub use self::running::running;
pub use self::style::{Style, RB_BOLD, RB_UNDERLINE, RB_REVERSE, RB_NORMAL};

use std::error::Error;
use std::io;
use std::fmt;
use std::char;
use std::time::duration::Duration;
use std::default::Default;

use termbox::RawEvent;
use libc::c_int;

pub mod keyboard;

pub use keyboard::Key;

#[derive(Clone, Copy)]
pub enum Event {
    KeyEventRaw(u8, u16, u32),
    KeyEvent(Option<Key>),
    ResizeEvent(i32, i32),
    NoEvent
}

#[derive(Clone, Copy, Debug)]
pub enum InputMode {
    Current = 0x00,

    /// When ESC sequence is in the buffer and it doesn't match any known
    /// ESC sequence => ESC means TB_KEY_ESC
    Esc     = 0x01,
    /// When ESC sequence is in the buffer and it doesn't match any known
    /// sequence => ESC enables TB_MOD_ALT modifier for the next keyboard event.
    Alt     = 0x02,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C,u16)]
pub enum Color {
    Default =  0x00,
    Black =    0x01,
    Red =      0x02,
    Green =    0x03,
    Yellow =   0x04,
    Blue =     0x05,
    Magenta =  0x06,
    Cyan =     0x07,
    White =    0x08,
}

mod style {
    bitflags! {
        #[repr(C)]
        flags Style: u16 {
            const TB_NORMAL_COLOR = 0x000F,
            const RB_BOLD = 0x0100,
            const RB_UNDERLINE = 0x0200,
            const RB_REVERSE = 0x0400,
            const RB_NORMAL = 0x0000,
            const TB_ATTRIB = RB_BOLD.bits | RB_UNDERLINE.bits | RB_REVERSE.bits,
        }
    }

    impl Style {
        pub fn from_color(color: super::Color) -> Style {
            Style { bits: color as u16 & TB_NORMAL_COLOR.bits }
        }
    }
}

const NIL_RAW_EVENT: RawEvent = RawEvent { etype: 0, emod: 0, key: 0, ch: 0, w: 0, h: 0, x: 0, y: 0 };

/// Unpack a RawEvent to an Event
///
/// if the `raw` parameter is true, then the Event variant will be the raw
/// representation of the event.
///     for instance KeyEventRaw instead of KeyEvent
///
/// This is useful if you want to interpret the raw event data yourself, rather
/// than having rustbox translate it to its own representation.
fn unpack_event(ev_type: c_int, ev: &RawEvent, raw: bool) -> io::Result<Event> {
    match ev_type {
        0 => Ok(Event::NoEvent),
        1 => Ok(
            if raw {
                Event::KeyEventRaw(ev.emod, ev.key, ev.ch)
            } else {
                let k = match ev.key {
                    0 => char::from_u32(ev.ch).map(|c| Key::Char(c)),
                    a => Key::from_code(a),
                };
                Event::KeyEvent(k)
            }),
        2 => Ok(Event::ResizeEvent(ev.w, ev.h)),
        -1 => Err(io::Error::last_os_error()),
        _ => panic!("Unsupported event"),
    }
}

#[derive(Clone, Copy, Debug)]
pub enum InitError {
    AlreadyOpen,
    UnsupportedTerminal,
    FailedToOpenTty,
    PipeTrapError,
}

impl InitError {
    fn from_termbox_error(res: i32) -> Self {
        use InitError::*;
        match res {
            -1 => UnsupportedTerminal,
            -2 => FailedToOpenTty,
            -3 => PipeTrapError,
            _ => panic!("Unhandled termbox init error: {}", res),
        }
    }
}

impl fmt::Display for InitError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.description())
    }
}

impl Error for InitError {
    fn description(&self) -> &str {
        use InitError::*;
        match *self {
            AlreadyOpen => "RustBox is already open.",
            UnsupportedTerminal => "Unsupported terminal.",
            FailedToOpenTty => "Failed to open TTY.",
            PipeTrapError => "Pipe trap error.",
        }
    }
}

mod running {
    use std::sync::atomic::{self, AtomicBool};

    // The state of the RustBox is protected by the lock.  Yay, global state!
    static RUSTBOX_RUNNING: AtomicBool = atomic::ATOMIC_BOOL_INIT;

    /// true iff RustBox is currently running.  Beware of races here--don't rely on this for anything
    /// critical unless you happen to know that RustBox cannot change state when it is called (a good
    /// usecase would be checking to see if it's worth risking double printing backtraces to avoid
    /// having them swallowed up by RustBox).
    pub fn running() -> bool {
        RUSTBOX_RUNNING.load(atomic::Ordering::SeqCst)
    }

    // Internal RAII guard used to ensure we release the running lock whenever we acquire it.
    pub struct RunningGuard;

    pub fn run() -> Option<RunningGuard> {
        // Ensure that we are not already running and simultaneously set RUSTBOX_RUNNING using an
        // atomic swap.  This ensures that contending threads don't trample each other.
        if RUSTBOX_RUNNING.swap(true, atomic::Ordering::SeqCst) {
            // The Rustbox was already running.
            None
        } else {
            // The RustBox was not already running, and now we have the lock.
            Some(RunningGuard)
        }
    }

    impl Drop for RunningGuard {
        fn drop(&mut self) {
            // Indicate that we're free now.  We could probably get away with lower atomicity here,
            // but there's no reason to take that chance.
            RUSTBOX_RUNNING.store(false, atomic::Ordering::SeqCst);
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct RustBox {
    // RAII lock.
    //
    // Note that running *MUST* be the last field in the destructor, since destructors run in
    // top-down order.  Otherwise it will not properly protect the above fields.
    _running: running::RunningGuard,
}

#[derive(Clone, Copy,Debug)]
pub struct InitOptions {
    /// Use this option to initialize with a specific input mode
    ///
    /// See InputMode enum for details on the variants.
    pub input_mode: InputMode,
}

impl Default for InitOptions {
    fn default() -> Self {
        InitOptions {
            input_mode: InputMode::Current,
        }
    }
}

impl RustBox {
    /// Initialize rustbox.
    ///
    /// For the default options, you can use:
    ///
    /// ```
    /// use rustbox::RustBox;
    /// use std::default::Default;
    /// let rb = RustBox::init(Default::default());
    /// ```
    ///
    /// Otherwise, you can specify:
    ///
    /// ```
    /// use rustbox::{RustBox, InitOptions};
    /// use std::default::Default;
    /// let rb = RustBox::init(InitOptions { input_mode: rustbox::InputMode::Esc, ..Default::default() });
    /// ```
    pub fn init(opts: InitOptions) -> Result<RustBox, InitError> {
        // Acquire RAII lock.  This might seem like overkill, but it is easy to forget to release
        // it in the maze of error conditions below.
        let running = match running::run() {
            Some(r) => r,
            None => return Err(InitError::AlreadyOpen)
        };

        // Create the RustBox.
        let mut rb = match unsafe { termbox::tb_init() } {
            0 => RustBox {
                _running: running,
            },
            res => {
                return Err(InitError::from_termbox_error(res));
            }
        };
        match opts.input_mode {
            InputMode::Current => (),
            _ => rb.set_input_mode(opts.input_mode),
        }
        Ok(rb)
    }

    pub fn width(&self) -> usize {
        unsafe { termbox::tb_width() as usize }
    }

    pub fn height(&self) -> usize {
        unsafe { termbox::tb_height() as usize }
    }

    pub fn clear(&mut self) {
        unsafe { termbox::tb_clear() }
    }

    pub fn present(&mut self) {
        unsafe { termbox::tb_present() }
    }

    pub fn set_cursor(&mut self, x: isize, y: isize) {
        unsafe { termbox::tb_set_cursor(x as c_int, y as c_int) }
    }

    pub unsafe fn change_cell(&mut self, x: usize, y: usize, ch: u32, fg: u16, bg: u16) {
        termbox::tb_change_cell(x as c_int, y as c_int, ch, fg, bg)
    }

    pub fn print(&mut self, x: usize, y: usize, sty: Style, fg: Color, bg: Color, s: &str) {
        let fg = Style::from_color(fg) | (sty & style::TB_ATTRIB);
        let bg = Style::from_color(bg);
        for (i, ch) in s.chars().enumerate() {
            unsafe {
                self.change_cell(x+i, y, ch as u32, fg.bits(), bg.bits());
            }
        }
    }

    pub fn print_char(&mut self, x: usize, y: usize, sty: Style, fg: Color, bg: Color, ch: char) {
        let fg = Style::from_color(fg) | (sty & style::TB_ATTRIB);
        let bg = Style::from_color(bg);
        unsafe {
            self.change_cell(x, y, ch as u32, fg.bits(), bg.bits());
        }
    }

    pub fn poll_event(&mut self, raw: bool) -> io::Result<Event> {
        let ev = NIL_RAW_EVENT;
        let rc = unsafe {
            termbox::tb_poll_event(&ev as *const RawEvent)
        };
        unpack_event(rc, &ev, raw)
    }

    pub fn peek_event(&mut self, timeout: Duration, raw: bool) -> io::Result<Event> {
        let ev = NIL_RAW_EVENT;
        let rc = unsafe {
            termbox::tb_peek_event(&ev as *const RawEvent, timeout.num_milliseconds() as c_int)
        };
        unpack_event(rc, &ev, raw)
    }

    pub fn set_input_mode(&mut self, mode: InputMode) {
        unsafe {
            termbox::tb_select_input_mode(mode as c_int);
        }
    }
}

impl Drop for RustBox {
    fn drop(&mut self) {
        // Since only one instance of the RustBox is ever accessible, we should not
        // need to do this atomically.
        // Note: we should definitely have RUSTBOX_RUNNING = true here.
        unsafe {
            termbox::tb_shutdown();
        }
    }
}

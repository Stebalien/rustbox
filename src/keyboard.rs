#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Key {
    Char(char),
    Key(u16),
}

impl Key {
    pub fn control(ch: char) -> Option<Key> {
        if ch as u32 > 0xFF {
            return None
        }
        Some(Key::Key(match ch {
            '2'|'~' => 0,
            'a'...'z' => ch as u16 - 'a' as u16 + 1,
            'A'...'Z' => ch as u16 - 'A' as u16 + 1,
            '3'|'[' => 0x1b,
            '4'|'\\' => 0x1c,
            '5'|']' => 0x1d,
            '6' => 0x1e,
            '7'|'/'|'_' => 0x1f,
            '8' => 0x7f,
            _ => return None,
        }))
    }

    pub fn funcion(num: u32) -> Option<Key> {
        if 1 <= num && num <= 12 {
            Some(Key::Key(0xFFFF - num as u16 - 1))
        } else {
            None
        }
    }
}

pub mod key {
    use super::Key;

    pub const F1: Key = Key::Key((0xFFFF-0));
    pub const F2: Key = Key::Key((0xFFFF-1));
    pub const F3: Key = Key::Key((0xFFFF-2));
    pub const F4: Key = Key::Key((0xFFFF-3));
    pub const F5: Key = Key::Key((0xFFFF-4));
    pub const F6: Key = Key::Key((0xFFFF-5));
    pub const F7: Key = Key::Key((0xFFFF-6));
    pub const F8: Key = Key::Key((0xFFFF-7));
    pub const F9: Key = Key::Key((0xFFFF-8));
    pub const F10: Key = Key::Key((0xFFFF-9));
    pub const F11: Key = Key::Key((0xFFFF-10));
    pub const F12: Key = Key::Key((0xFFFF-11));
    pub const INSERT: Key = Key::Key((0xFFFF-12));
    pub const DELETE: Key = Key::Key((0xFFFF-13));
    pub const HOME: Key = Key::Key((0xFFFF-14));
    pub const END: Key = Key::Key((0xFFFF-15));
    pub const PGUP: Key = Key::Key((0xFFFF-16));
    pub const PGDN: Key = Key::Key((0xFFFF-17));
    pub const ARROW_UP: Key = Key::Key((0xFFFF-18));
    pub const ARROW_DOWN: Key = Key::Key((0xFFFF-19));
    pub const ARROW_LEFT: Key = Key::Key((0xFFFF-20));
    pub const ARROW_RIGHT: Key = Key::Key((0xFFFF-21));
    pub const MOUSE_LEFT: Key = Key::Key((0xFFFF-22));
    pub const MOUSE_RIGHT: Key = Key::Key((0xFFFF-23));
    pub const MOUSE_MIDDLE: Key = Key::Key((0xFFFF-24));
    pub const MOUSE_RELEASE: Key = Key::Key((0xFFFF-25));
    pub const MOUSE_WHEEL_UP: Key = Key::Key((0xFFFF-26));
    pub const MOUSE_WHEEL_DOWN: Key = Key::Key((0xFFFF-27));

    pub const CTRL_TILDE: Key = Key::Key(0x00);
    pub const CTRL_2: Key = Key::Key(0x00); /* clash with 'CTRL_TILDE' */
    pub const CTRL_A: Key = Key::Key(0x01);
    pub const CTRL_B: Key = Key::Key(0x02);
    pub const CTRL_C: Key = Key::Key(0x03);
    pub const CTRL_D: Key = Key::Key(0x04);
    pub const CTRL_E: Key = Key::Key(0x05);
    pub const CTRL_F: Key = Key::Key(0x06);
    pub const CTRL_G: Key = Key::Key(0x07);
    pub const BACKSPACE: Key = Key::Key(0x08);
    pub const CTRL_H: Key = Key::Key(0x08); /* clash with 'CTRL_BACKSPACE' */
    pub const TAB: Key = Key::Key(0x09);
    pub const CTRL_I: Key = Key::Key(0x09); /* clash with 'TAB' */
    pub const CTRL_J: Key = Key::Key(0x0A);
    pub const CTRL_K: Key = Key::Key(0x0B);
    pub const CTRL_L: Key = Key::Key(0x0C);
    pub const ENTER: Key = Key::Key(0x0D);
    pub const CTRL_M: Key = Key::Key(0x0D); /* clash with 'ENTER' */
    pub const CTRL_N: Key = Key::Key(0x0E);
    pub const CTRL_O: Key = Key::Key(0x0F);
    pub const CTRL_P: Key = Key::Key(0x10);
    pub const CTRL_Q: Key = Key::Key(0x11);
    pub const CTRL_R: Key = Key::Key(0x12);
    pub const CTRL_S: Key = Key::Key(0x13);
    pub const CTRL_T: Key = Key::Key(0x14);
    pub const CTRL_U: Key = Key::Key(0x15);
    pub const CTRL_V: Key = Key::Key(0x16);
    pub const CTRL_W: Key = Key::Key(0x17);
    pub const CTRL_X: Key = Key::Key(0x18);
    pub const CTRL_Y: Key = Key::Key(0x19);
    pub const CTRL_Z: Key = Key::Key(0x1A);
    pub const ESC: Key = Key::Key(0x1B);
    pub const CTRL_LSQ_BRACKET: Key = Key::Key(0x1B); /* clash with 'ESC' */
    pub const CTRL_3: Key = Key::Key(0x1B); /* clash with 'ESC' */
    pub const CTRL_4: Key = Key::Key(0x1C);
    pub const CTRL_BACKSLASH: Key = Key::Key(0x1C); /* clash with 'CTRL_4' */
    pub const CTRL_5: Key = Key::Key(0x1D);
    pub const CTRL_RSQ_BRACKET: Key = Key::Key(0x1D); /* clash with 'CTRL_5' */
    pub const CTRL_6: Key = Key::Key(0x1E);
    pub const CTRL_7: Key = Key::Key(0x1F);
    pub const CTRL_SLASH: Key = Key::Key(0x1F); /* clash with 'CTRL_7' */
    pub const CTRL_UNDERSCORE: Key = Key::Key(0x1F); /* clash with 'CTRL_7' */
    pub const SPACE: Key = Key::Key(0x20);
    pub const BACKSPACE2: Key = Key::Key(0x7F);
    pub const CTRL_8: Key = Key::Key(0x7F); /* clash with 'DELETE' */
}

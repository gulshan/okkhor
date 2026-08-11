//! Converting a word while it is still being typed.
//!
//! [`Parser`] converts a finished word. An input method has to show that word
//! as it grows, which means re-converting after every keystroke and patching
//! the difference onto text the user can already see. [`Editor`] holds the
//! buffer, remembers what it last put on screen, and reports the smallest
//! change that brings the two back into agreement.
//!
//! # Driving it
//!
//! If a keystroke produces a character, give it to [`Editor::put_char`].
//! Otherwise — arrow keys, function keys, a mouse click, the window losing
//! focus — call [`Editor::put_non_char`]. Either way, read the returned
//! [`Edit`]:
//!
//! - [`Edit::is_empty`] — the editor did not take the keystroke. Let it reach
//!   the application unchanged.
//! - anything else — the editor took it. Swallow the keystroke, erase
//!   [`Edit::backspaces`] characters from the end of the text on screen, and
//!   type [`Edit::output`] in their place.
//!
//! ```
//! # use okkhor::editor::Editor;
//! let mut editor = Editor::new_phonetic();
//!
//! assert_eq!(editor.put_char('k').output, "ক");
//!
//! // k and x together are ক্ষ, so the ক already on screen is kept and only
//! // the rest of the conjunct is typed after it.
//! let edit = editor.put_char('x');
//! assert_eq!(edit.backspaces, 0);
//! assert_eq!(edit.output, "\u{09CD}\u{09B7}");
//! ```
//!
//! # What the editor assumes
//!
//! That its own output is still the last thing on screen, and that the caret is
//! still sitting after it. The editor cannot see the caret, so holding up that
//! assumption is the caller's job: anything that might move it — a click, a
//! focus change, a word ending — has to be reported with
//! [`Editor::put_non_char`], or the next edit will erase text it does not own.

use crate::parser::Parser;

/// The smallest change that turns what is on screen into what should be there.
///
/// Erase the stale tail from the end of the text, then type [`Edit::output`].
/// The two counts describe that same tail in different units; use whichever
/// suits the thing being edited.
#[derive(Default)]
pub struct Edit<'a> {
    /// Length of the stale tail in UTF-8 bytes, for editing a Rust [`String`]
    /// with [`String::truncate`].
    pub byte_backspaces: usize,
    /// Length of the stale tail in `char`s — the number of backspaces to send
    /// to a text field.
    ///
    /// This is also its length in UTF-16 code units, which is what a Windows
    /// edit control counts: one `VK_BACK` removes one UTF-16 unit. The two
    /// agree because the parser takes only ASCII and everything it can emit
    /// lies in the basic multilingual plane, so no character it produces needs
    /// a surrogate pair.
    pub backspaces: usize,
    /// What to type once the stale tail has been erased. Can be empty while
    /// [`Edit::backspaces`] is not — see [`Edit::is_empty`].
    pub output: &'a str,
}

impl Edit<'_> {
    /// Nothing to do: the editor did not take the keystroke, so let it through.
    ///
    /// Both fields have to be tested. An ordinary keystroke that only appends
    /// erases nothing, so `backspaces == 0` does not mean idle; and a backtick
    /// can erase without typing anything — `` o` `` removes the `অ` that `o`
    /// produced and puts nothing in its place — so an empty `output` does not
    /// mean idle either.
    pub fn is_empty(&self) -> bool {
        self.backspaces == 0 && self.output.is_empty()
    }
}

/// A parser, the romanised word typed so far, and the converted text the
/// editor believes is currently on screen.
pub struct Editor {
    parser: Parser,
    input_buffer: String,
    output: String,
}

impl Editor {
    /// An editor over the phonetic (Avro-style) parser.
    pub fn new_phonetic() -> Editor {
        Editor {
            parser: Parser::new_phonetic(),
            input_buffer: String::new(),
            output: String::new(),
        }
    }

    /// Forget the current word without touching the screen.
    fn reset(&mut self) {
        self.input_buffer.clear();
        self.output.clear();
    }

    /// Extend the current word with a typed character.
    ///
    /// Anything that cannot be part of a romanised word ends the word instead
    /// and returns an empty [`Edit`], so the keystroke passes straight through:
    /// non-ASCII characters, whitespace, and control characters such as the
    /// U+0008 and U+001B that a key-to-character translation yields for
    /// Backspace and Escape. Every `char` is therefore safe to pass, and the
    /// returned edit says whether it was taken — the caller does not have to
    /// know in advance which keys the parser can use.
    ///
    /// Note that a word is only ended, never committed: whatever is on screen
    /// stays there.
    pub fn put_char<'a>(&'a mut self, new_ch: char) -> Edit<'a> {
        if !new_ch.is_ascii_graphic() {
            self.reset();
            Edit::default()
        } else {
            self.input_buffer.push(new_ch);
            let output = self.parser.convert(&self.input_buffer);

            let mut at = self
                .output
                .bytes()
                .zip(output.bytes())
                .take_while(|(a, b)| a == b)
                .count();
            while at > 0 && !self.output.is_char_boundary(at) {
                at -= 1;
            }

            let byte_backspaces = self.output.len() - at;
            let backspaces = self.output[at..].chars().count();
            self.output = output;
            Edit {
                byte_backspaces,
                backspaces,
                output: &self.output[at..],
            }
        }
    }

    /// Report something the editor cannot convert: Backspace, Enter, Delete,
    /// an arrow key, a mouse click, the window losing focus.
    ///
    /// The current word is abandoned so the next character starts a fresh one,
    /// and what is already on screen is left alone — the editor simply stops
    /// claiming it may edit it.
    ///
    /// The returned [`Edit`] is always empty, since the keystroke is the
    /// caller's to deliver. It is returned rather than omitted so that both
    /// methods can feed one call site.
    pub fn put_non_char<'a>(&'a mut self) -> Edit<'a> {
        self.reset();
        Edit::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_test() {
        let mut editor = Editor::new_phonetic();

        editor.put_char('k');
        assert_eq!("ক", editor.output);

        editor.put_char('o');
        assert_eq!("ক", editor.output);

        editor.put_char('r');
        assert_eq!("কর", editor.output);

        editor.put_char('r');
        assert_eq!("করর", editor.output);

        editor.put_char('m');
        assert_eq!("কর্ম", editor.output);

        editor.put_char(' ');
        assert_eq!("", editor.output);
    }

    /// Type `keys`, applying every edit to a model of the target's screen, and
    /// assert the screen still matches the parser after each keystroke.
    fn type_word(editor: &mut Editor, keys: &str) -> String {
        let parser = Parser::new_phonetic();
        let mut raw = String::new();
        let mut screen = String::new();

        for key in keys.chars() {
            raw.push(key);
            let edit = editor.put_char(key);

            let keep = screen.len() - edit.byte_backspaces;
            screen.truncate(keep);
            screen.push_str(edit.output);

            assert_eq!(screen, parser.convert(&raw), "after {key:?} of {keys:?}");
        }
        screen
    }

    #[test]
    fn converges_on_the_parser() {
        for word in ["ami", "banglay", "kkhoma", "kxoma", "bangladesh", "3.14"] {
            let mut editor = Editor::new_phonetic();
            assert_eq!(
                type_word(&mut editor, word),
                Parser::new_phonetic().convert(word)
            );
        }
    }

    #[test]
    fn a_vowel_sign_lands_on_the_base_already_on_screen() {
        let mut editor = Editor::new_phonetic();
        editor.put_char('a');
        editor.put_char('m');
        let edit = editor.put_char('i');
        assert_eq!(edit.backspaces, 0);
        assert_eq!(edit.output, "ি");
    }

    #[test]
    fn a_reanalysed_conjunct_rewrites_only_its_tail() {
        let mut editor = Editor::new_phonetic();
        assert_eq!(editor.put_char('k').output, "ক");
        // ক -> ক্ষ keeps the ক and appends hasant + ষ.
        let edit = editor.put_char('x');
        assert_eq!(edit.backspaces, 0);
        assert_eq!(edit.output, "\u{09CD}\u{09B7}");
    }

    #[test]
    fn erasing_is_bounded_by_the_shared_prefix() {
        // The pathological case for a full rewrite: a long word rewritten on
        // every keystroke. Nothing here may erase more than a few units.
        let mut editor = Editor::new_phonetic();
        let mut worst = 0;
        for ch in "banglakobitarboi".chars() {
            worst = worst.max(editor.put_char(ch).backspaces);
        }
        assert!(worst <= 2, "worst erase was {worst}");
    }

    #[test]
    fn reset_leaves_the_screen_alone() {
        let mut editor = Editor::new_phonetic();
        editor.put_char('a');
        editor.put_char('m');
        editor.put_non_char();
        assert!(editor.output.is_empty());
        assert_eq!(editor.output, "");
        // The next word starts from nothing, and does not erase the last one.
        assert_eq!(editor.put_char('i').backspaces, 0);
    }

    #[test]
    fn a_word_never_reaches_back_across_a_reset() {
        // kx is ক্ষ only because the k is still buffered. This is what the
        // caller buys by calling put_non_char() at a word break.
        let mut editor = Editor::new_phonetic();
        editor.put_char('k');
        editor.put_non_char();
        assert_eq!(editor.put_char('x').output, "এক্স");
    }

    /// The sequence from okkhor's existing editor test, without the caret
    /// positions the old API required.
    #[test]
    fn matches_the_previous_editor_behaviour() {
        let mut editor = Editor::new_phonetic();
        for (ch, expected) in [
            ('k', "ক"),
            ('o', "ক"),
            ('r', "কর"),
            ('r', "করর"),
            ('m', "কর্ম"),
        ] {
            editor.put_char(ch);
            assert_eq!(editor.output, expected, "after {ch:?}");
        }
        editor.put_non_char();
        assert_eq!(editor.output, "");
    }

    /// The one case where the output is empty and there is still work to do.
    /// A backtick is Avro's "do not combine" escape, and after a lone `o` it
    /// undoes the conversion outright.
    ///
    /// This is why `Edit::is_empty` tests both fields. A caller keying on
    /// `output.is_empty()` alone would treat this as a passthrough, leaving the
    /// অ on screen with a stray backtick after it.
    #[test]
    fn an_escape_can_erase_without_typing_anything() {
        let mut editor = Editor::new_phonetic();
        assert_eq!(editor.put_char('o').output, "অ");

        let edit = editor.put_char('`');
        assert_eq!(edit.backspaces, 1);
        assert_eq!(edit.output, "");
        assert!(!edit.is_empty(), "there is still an অ to erase");
    }

    /// The two counts describe the same tail, so they have to agree on it:
    /// `অ` is one code point and three UTF-8 bytes.
    #[test]
    fn the_two_counts_measure_the_same_tail() {
        let mut editor = Editor::new_phonetic();
        editor.put_char('o');

        let edit = editor.put_char('`');
        assert_eq!(edit.backspaces, 1);
        assert_eq!(edit.byte_backspaces, 3);
    }

    /// A space ends the word and is not consumed. The caller has to pass it on,
    /// or it is lost from the text.
    #[test]
    fn an_empty_edit_means_the_keystroke_was_not_taken() {
        let mut editor = Editor::new_phonetic();
        for ch in "ami".chars() {
            editor.put_char(ch);
        }

        let edit = editor.put_char(' ');
        assert!(edit.is_empty());
        assert_eq!(edit.backspaces, 0);
        assert_eq!(edit.byte_backspaces, 0);
        assert_eq!(edit.output, "");
    }

    #[test]
    fn put_non_char_always_yields_an_empty_edit() {
        let mut editor = Editor::new_phonetic();
        assert!(editor.put_non_char().is_empty());

        editor.put_char('k');
        assert!(editor.put_non_char().is_empty());
    }

    /// The parser byte-indexes its input and panics on anything multi-byte, so
    /// the guard in `put_char` is what stops a stray character taking the whole
    /// process down. A caller cannot always predict what a keystroke produces —
    /// a dead key or an AltGr combination can yield an accented letter.
    #[test]
    fn non_ascii_is_refused_rather_than_panicking() {
        for ch in ['é', 'অ', '\u{1F600}'] {
            let mut editor = Editor::new_phonetic();
            editor.put_char('a');

            assert!(editor.put_char(ch).is_empty(), "{ch:?} should be ignored");
            assert_eq!(editor.output, "", "{ch:?} should have ended the word");
        }
    }

    /// On Windows, translating a keystroke to a character gives U+0008 for
    /// Backspace, U+001B for Escape and U+007F for Delete. None can be part of
    /// a word, and letting one into the buffer both corrupts every later
    /// conversion and emits a control character into the user's text.
    #[test]
    fn control_characters_never_enter_the_buffer() {
        for ch in ['\u{8}', '\u{1b}', '\u{7f}'] {
            let mut editor = Editor::new_phonetic();
            editor.put_char('a');

            assert!(editor.put_char(ch).is_empty(), "{ch:?} should be ignored");
            assert_eq!(editor.output, "", "{ch:?} should have ended the word");
        }
    }

    /// Punctuation is converted too, and the multi-character patterns only fire
    /// when both characters are in the buffer together, so these exercise the
    /// preview correcting itself mid-word.
    #[test]
    fn punctuation_and_escapes_survive_the_live_preview() {
        for keys in ["ami.", "ami..", "bhalo:", "100$", "ka,,kha", "3.14", "o`"] {
            let mut editor = Editor::new_phonetic();
            assert_eq!(
                type_word(&mut editor, keys),
                Parser::new_phonetic().convert(keys),
                "typing {keys:?}"
            );
        }
    }
}

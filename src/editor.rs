use crate::parser::Parser;

pub struct PhoneticEditor {
    input_buffer: String,
    last_output_len: usize,
    last_cursor_pos: usize,
    parser: Parser,
}

#[derive(Debug)]
pub struct EditorResult {
    pub output: String,
    pub replace_len: usize,
}

impl Default for PhoneticEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl PhoneticEditor {
    pub fn new() -> PhoneticEditor {
        PhoneticEditor {
            input_buffer: String::new(),
            last_output_len: 0,
            last_cursor_pos: 0,
            parser: Parser::new_phonetic(),
        }
    }

    pub fn reset(&mut self) {
        self.input_buffer.clear();
        self.last_output_len = 0;
        self.last_cursor_pos = 0;
    }

    pub fn put_new_ch(&mut self, new_ch: char, cursor_pos: usize) -> EditorResult {
        if new_ch.is_ascii_whitespace() {
            self.reset();
            EditorResult {
                output: String::default(),
                replace_len: 0,
            }
        } else {
            if cursor_pos == self.last_cursor_pos + 1 {
                self.input_buffer.push(new_ch)
            } else {
                self.input_buffer.clear();
                self.input_buffer.push(new_ch);
                self.last_output_len = 0
            }

            let result = EditorResult {
                output: self.parser.convert(&self.input_buffer),
                replace_len: self.last_output_len + 1,
            };

            self.last_output_len = result.output.chars().count();
            self.last_cursor_pos = cursor_pos - result.replace_len + self.last_output_len;

            result
        }
    }
}

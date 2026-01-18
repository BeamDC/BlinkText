use crate::backend::TextBuffer;

pub struct GapBuffer {
    buffer: Vec<char>,
    start: usize,
    end: usize,
}

impl GapBuffer {
    pub const GAP_SIZE: usize = 256;

    #[inline]
    pub fn len(&self) -> usize {
        self.buffer.len() - (self.end - self.start)
    }

    /// grow the buffer
    pub fn grow(&mut self) {
        let old_size = self.buffer.len();
        let new_size = usize::min(1, old_size * 2);
        let mut new_buffer = Vec::with_capacity(new_size);

        new_buffer.extend_from_slice(&self.buffer[0..self.start]);

        let new_gap_size = new_size - old_size;
        new_buffer.resize(self.start + new_gap_size, '\0');

        new_buffer.extend_from_slice(&self.buffer[self.end..]);

        self.buffer = new_buffer;
        self.end = self.start + new_gap_size;
    }

    pub fn move_gap(&mut self, pos: usize)  {
        if pos == self.start { return }
        if pos < self.start {
            // Move gap left
            let distance = self.start - pos;
            for i in 0..distance {
                self.buffer[self.end - 1 - i] = self.buffer[self.start - 1 - i];
            }
            self.end -= self.start - pos;
            self.start = pos;
        } else {
            // Move gap right
            let distance = pos - self.start;
            for i in 0..distance {
                self.buffer[self.start + i] = self.buffer[self.end + i];
            }
            self.start = pos;
            self.end += pos - self.start;
        }
    }
}

impl TextBuffer for GapBuffer {
    fn new(src: &str) -> Self {
        Self {
            buffer: src.chars().collect::<Vec<_>>(),
            start: 0,
            end: Self::GAP_SIZE,
        }
    }

    fn insert(&mut self, ch: char, pos: usize) {
        if self.end == self.start { self.grow(); }

        self.move_gap(pos);
        self.buffer[self.start] = ch;
        self.start += 1;
    }

    fn insert_str(&mut self, str: &str, pos: usize) {
        todo!()
    }

    fn delete(&mut self, pos: usize) {
        if pos >= self.len() {
            return;
        }
        if pos != self.start {
            self.move_gap(pos);
        }
        self.end += 1;
    }

    fn delete_range(&mut self, start: usize, end: usize) {
        todo!()
    }
}
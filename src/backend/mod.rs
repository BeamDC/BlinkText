mod gap_buffer;

/// a trait for any type which can be used to store and edit the contents of a file
pub trait TextBuffer: Sized {
    /// construct self from a string
    fn new(src: &str) -> Self;
    /// insert a char at the given position
    fn insert(&mut self, ch: char, pos: usize);
    /// insert a whole string at the given position
    fn insert_str(&mut self, str: &str, pos: usize);
    /// remove the char at the given position
    fn delete(&mut self, pos: usize);
    /// remove all chars in the given range
    fn delete_range(&mut self, start: usize, end: usize);
}
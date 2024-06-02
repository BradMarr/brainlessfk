pub trait CharExtensions {
    fn to_ascii(&self) -> char;
}

impl CharExtensions for char {
    /// Converts char to ascii.
    fn to_ascii(&self) -> char {
        if self.is_lowercase() {
            return self.to_ascii_lowercase();
        } else {
            return self.to_ascii_uppercase();
        }
    }
}
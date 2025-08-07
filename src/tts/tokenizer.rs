// Simple tokenizer for KittenTTS
// This is a basic implementation - in a full implementation you'd want
// to match the exact tokenization used during training

pub fn tokenize(text: &str) -> Vec<String> {
    text.chars()
        .filter(|c| !c.is_whitespace() || *c == ' ')
        .map(|c| c.to_string())
        .collect()
}
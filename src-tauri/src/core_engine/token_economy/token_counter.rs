/// Heuristic token counter — ~4 chars per token for English text.
/// For production, swap in tiktoken-rs or similar.
pub struct TokenCounter;

impl TokenCounter {
    pub fn estimate(text: &str) -> u32 {
        ((text.len() as f64) / 4.0).ceil() as u32
    }
}

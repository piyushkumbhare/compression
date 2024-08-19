// Contains the definition for the Encoder trait

/// This trait requires 2 functions (encode & decode) that both map from a &str to a String
pub trait Encoder {
    fn encode(&self, s: &str) -> String;
    fn decode(&self, s: &str) -> String;
}
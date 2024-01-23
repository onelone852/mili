#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoverData {
    pub session_id: Box<str>,
    pub resume_url: Box<str>,
}

pub struct FSObj {
    pub name: String,
    pub meta: std::fs::Metadata,
    pub requested_type: String,
    pub mode: u32,
}

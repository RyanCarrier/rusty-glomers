pub struct State {
    pub node_ids: Vec<String>,
    pub seen_messages: Vec<usize>,
}

impl State {
    pub fn new() -> Self {
        State {
            node_ids: Vec::new(),
            seen_messages: Vec::new(),
        }
    }
}

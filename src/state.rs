use std::collections::HashMap;

pub struct State {
    pub node_id: String,
    pub node_ids: Vec<String>,
    pub seen_messages: Vec<usize>,
    pub topology: HashMap<String, Vec<String>>,
}

impl State {
    pub fn new() -> Self {
        State {
            node_id: String::from(""),
            node_ids: Vec::new(),
            seen_messages: Vec::new(),
            topology: HashMap::new(),
        }
    }
}

use log::info;

use crate::{
    message::{MaelstromMessage, MessageType},
    setup_logging,
};

pub struct State {
    pub node_id: String,
    pub node_ids: Vec<String>,
    pub seen_messages: Vec<usize>,
}

impl State {
    pub fn new() -> Self {
        State {
            node_id: String::from(""),
            node_ids: Vec::new(),
            seen_messages: Vec::new(),
        }
    }

    pub fn handle(&mut self, msg: MaelstromMessage) {
        match &msg.body.msg_type {
            MessageType::InitOk
            | MessageType::EchoOk
            | MessageType::GenerateOk
            | MessageType::ReadOk
            | MessageType::TopologyOk
            | MessageType::BroadcastOk => {}
            MessageType::Init => {
                self.node_id = msg.body.node_id.clone().unwrap();
                self.node_ids = msg
                    .body
                    .node_ids
                    .clone()
                    .unwrap()
                    .into_iter()
                    .filter(|id| id != &self.node_id)
                    .collect();
                setup_logging(Some(self.node_id.clone()));
                log::info!("init complete");
            }
            MessageType::Echo => {}
            MessageType::Generate => {}
            MessageType::Broadcast => {
                info!(
                    "Broadcast recieved: {} from {}",
                    &msg.body.message.unwrap(),
                    &msg.src,
                );
                let message: usize = msg.body.message.unwrap().clone();
                if !self.seen_messages.contains(&message) {
                    self.seen_messages.push(message);
                    if !self.node_ids.contains(&msg.src) {
                        MaelstromMessage::broadcast_msgs(&self, &msg);
                    }
                }
            }
            MessageType::Read | MessageType::Topology => {}
        }
        match msg.get_response(self) {
            Ok(r) => r.post(),
            Err(_) => {}
        }
        // self.repost_on_late();
    }
}

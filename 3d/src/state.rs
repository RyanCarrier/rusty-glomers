use std::collections::HashMap;

use log::info;

use crate::{
    message::{MaelstromMessage, MessageType},
    setup_logging,
};

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

    pub fn handle(&mut self, msg: MaelstromMessage) {
        match &msg.body.msg_type {
            MessageType::InitOk
            | MessageType::EchoOk
            | MessageType::GenerateOk
            | MessageType::ReadOk
            | MessageType::TopologyOk
            | MessageType::BroadcastOk => {}
            MessageType::Init => {
                self.node_ids = msg.body.node_ids.clone().unwrap();
                self.node_id = msg.body.node_id.clone().unwrap();
                setup_logging(Some(self.node_id.clone()));
                log::info!("init complete");
            }
            MessageType::Echo => {}
            MessageType::Generate => {}
            MessageType::Broadcast => {
                info!(
                    "Broadcast recieved: {} from {}",
                    &msg.body.message.unwrap(),
                    &msg.src
                );
                let message: usize = msg.body.message.clone().unwrap();
                if !self.seen_messages.contains(&message) {
                    self.seen_messages.push(message);
                    //broadcast new message to friends
                    MaelstromMessage::get_broadcast_msg(&self, &msg)
                        .into_iter()
                        .for_each(|x| x.post());
                }
            }
            MessageType::Read => {}
            MessageType::Topology => {
                self.topology = msg.body.topology.clone().unwrap();
                info!("Topology: {:?}", self.topology);
            }
        }
        match msg.get_response(self) {
            Ok(r) => r.post(),
            Err(_) => {}
        }
        // self.repost_on_late();
    }
}

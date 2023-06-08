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
    pub locals: Vec<String>,
    pub broadcast_topology: HashMap<String, Vec<String>>,
}

impl State {
    pub fn new() -> Self {
        State {
            node_id: String::from(""),
            node_ids: Vec::new(),
            seen_messages: Vec::new(),
            topology: HashMap::new(),
            locals: Vec::new(),
            broadcast_topology: HashMap::new(),
        }
    }

    pub fn update_topology(&mut self, topology: HashMap<String, Vec<String>>) {
        self.topology = topology;
        self.locals = self.topology.get(&self.node_id).unwrap().clone();
        self.broadcast_topology.clear();
        self.node_ids.iter().for_each(|src_node| {
            let src_local_nodes: &Vec<String> = self.topology.get(src_node).unwrap();
            self.broadcast_topology.insert(
                src_node.clone(),
                self.locals
                    .clone()
                    .into_iter()
                    .filter(|x| !(src_local_nodes.contains(x) || x == src_node))
                    .collect(),
            );
        });
        info!("Topology: {:?}", self.topology);
        info!("Broadcast top: {:?}", self.broadcast_topology);
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
                let message: usize = msg.body.message.unwrap().clone();
                if !self.seen_messages.contains(&message) {
                    self.seen_messages.push(message);
                    MaelstromMessage::broadcast_msgs(&self, &msg);
                }
            }
            MessageType::Read => {}
            MessageType::Topology => self.update_topology(msg.body.topology.clone().unwrap()),
        }
        match msg.get_response(self) {
            Ok(r) => r.post(),
            Err(_) => {}
        }
        // self.repost_on_late();
    }
}

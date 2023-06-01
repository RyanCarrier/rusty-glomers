use std::{collections::HashMap, time::SystemTime};

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
    pub awaiting_ack: Vec<PostAck>,
}

impl State {
    pub const LOOP_DELAY_MS: u64 = 10;
    pub fn new() -> Self {
        State {
            awaiting_ack: Vec::new(),
            node_id: String::from(""),
            node_ids: Vec::new(),
            seen_messages: Vec::new(),
            topology: HashMap::new(),
        }
    }
    pub fn add_to_ack(&mut self, msg: MaelstromMessage) {
        self.awaiting_ack.push(PostAck {
            timestamp: SystemTime::now(),
            msg,
        });
    }
    pub fn try_remove_ack(&mut self, msg: &MaelstromMessage) {
        info!(
            "OK Recieved: {} to {} received OK",
            msg.body.msg_id.unwrap() % 10_000,
            msg.src
        );
        for i in 0..self.awaiting_ack.len() {
            if self.awaiting_ack[i].msg.dest == msg.src
                && self.awaiting_ack[i].msg.body.msg_id.unwrap() == msg.body.msg_id.unwrap()
            {
                self.awaiting_ack.remove(i);
                return;
            }
        }
    }

    pub fn post_ack(&mut self, msg: MaelstromMessage) {
        // todo: we need to put maelstromMessage into the awaiting ack
        self.add_to_ack(msg.clone());
        info!("PostAck: {}", msg);
        msg.post();
    }
    pub fn repost_on_late(&mut self) {
        info!("repost_on_late");
        // self.awaiting_ack.iter().for_each(|x| {
        //     info!("reposting: {}", x.msg);
        //     x.msg.post();
        // });
        let need_to_repost: Vec<PostAck>;
        (self.awaiting_ack, need_to_repost) =
            self.awaiting_ack.clone().into_iter().partition(|x| {
                (&x).timestamp.elapsed().unwrap().as_millis() < MaelstromMessage::REPOST_DELAY_MS
            });
        //       self.awaiting_ack = need_to_wait;
        need_to_repost.iter().for_each(|x| {
            info!(
                "Reposting: {} to {}",
                &x.msg.body.message.unwrap(),
                &x.msg.dest
            )
        });
        need_to_repost
            .into_iter()
            .for_each(|x| self.post_ack(x.msg));
    }

    pub fn handle(&mut self, msg: MaelstromMessage) {
        match &msg.body.msg_type {
            MessageType::InitOk
            | MessageType::EchoOk
            | MessageType::GenerateOk
            | MessageType::ReadOk
            | MessageType::TopologyOk => {}
            MessageType::BroadcastOk => self.try_remove_ack(&msg),
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
                    MaelstromMessage::get_broadcast_msg(&self, msg.clone())
                        .into_iter()
                        .for_each(|x| self.post_ack(x));
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

#[derive(Clone)]
pub struct PostAck {
    pub timestamp: SystemTime,
    pub msg: MaelstromMessage,
}

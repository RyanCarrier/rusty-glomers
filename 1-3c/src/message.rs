use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};
use serde_with;
use uuid::Uuid;

use crate::state::State;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MaelstromMessage {
    pub src: String,
    pub dest: String,
    pub body: MaelstromMessageBody,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MaelstromMessageBody {
    #[serde(rename = "type")]
    pub msg_type: MessageType,
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    pub echo: Option<String>,
    pub node_id: Option<String>,
    pub id: Option<String>,
    pub node_ids: Option<Vec<String>>,
    pub message: Option<usize>,
    pub messages: Option<Vec<usize>>,
    pub topology: Option<HashMap<String, Vec<String>>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Init,
    InitOk,
    Echo,
    EchoOk,
    Generate,
    GenerateOk,
    Broadcast,
    BroadcastOk,
    Read,
    ReadOk,
    Topology,
    TopologyOk,
}

impl fmt::Display for MaelstromMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl MaelstromMessage {
    pub const REPOST_DELAY_MS: u128 = 30;
    pub fn post(&self) {
        println!("{}", serde_json::to_string(self).unwrap());
    }
    pub fn get_broadcast_msg(state: &State, msg: MaelstromMessage) -> Vec<Self> {
        let temp: Vec<Self> = state
            .topology
            .get(&state.node_id)
            .unwrap()
            .iter()
            .filter(|x| **x != msg.src)
            .map(|dst_node| MaelstromMessage {
                src: state.node_id.clone(),
                dest: dst_node.clone(),
                body: MaelstromMessageBody::get_broadcast_body(&msg),
            })
            .collect();
        temp.iter()
            .for_each(|x| log::info!("Broadcasting {} to {}", msg.body.message.unwrap(), x.dest));
        temp
    }
    pub fn get_response(self, state: &State) -> Result<MaelstromMessage, String> {
        let body = self.body.get_response(state)?;
        Ok(MaelstromMessage {
            src: self.dest,
            dest: self.src,
            body,
        })
    }
}

impl MaelstromMessageBody {
    pub fn get_broadcast_body(msg: &MaelstromMessage) -> Self {
        let dest_id: usize = msg.dest[1..].parse().unwrap();
        let message: usize = msg.body.message.unwrap();
        let msg_id = (dest_id * 10_000) + message;
        MaelstromMessageBody {
            msg_type: MessageType::Broadcast,
            msg_id: Some(msg_id),
            in_reply_to: None,
            echo: None,
            node_id: None,
            id: None,
            node_ids: None,
            message: Some(message),
            messages: None,
            topology: None,
        }
    }
    pub fn get_response(self, state: &State) -> Result<MaelstromMessageBody, String> {
        match self.msg_type {
            MessageType::InitOk
            | MessageType::EchoOk
            | MessageType::TopologyOk
            | MessageType::GenerateOk
            | MessageType::ReadOk
            | MessageType::BroadcastOk => Err(String::from("can't handle response")),
            MessageType::Init => Ok(MaelstromMessageBody {
                node_id: None,
                topology: None,
                id: None,
                msg_type: MessageType::InitOk,
                msg_id: self.msg_id,
                in_reply_to: self.msg_id,
                echo: None,
                node_ids: None,
                message: None,
                messages: None,
            }),
            MessageType::Echo => Ok(MaelstromMessageBody {
                topology: None,
                node_id: None,
                id: None,
                msg_type: MessageType::EchoOk,
                msg_id: self.msg_id,
                in_reply_to: self.msg_id,
                echo: self.echo,
                node_ids: None,
                message: None,
                messages: None,
            }),

            MessageType::Generate => Ok(MaelstromMessageBody {
                topology: None,
                node_id: None,
                id: Some(Uuid::new_v4().to_string()),
                msg_type: MessageType::GenerateOk,
                msg_id: self.msg_id,
                in_reply_to: self.msg_id,
                echo: None,
                node_ids: None,
                message: None,
                messages: None,
            }),
            MessageType::Broadcast => Ok(MaelstromMessageBody {
                node_id: None,
                topology: None,
                id: None,
                msg_type: MessageType::BroadcastOk,
                msg_id: self.msg_id,
                in_reply_to: self.msg_id,
                echo: None,
                node_ids: None,
                message: None,
                messages: None,
            }),
            MessageType::Read => Ok(MaelstromMessageBody {
                topology: None,
                node_id: None,
                id: None,
                msg_type: MessageType::ReadOk,
                msg_id: self.msg_id,
                in_reply_to: self.msg_id,
                echo: None,
                node_ids: None,
                message: None,
                messages: Some(state.seen_messages.clone()),
            }),
            MessageType::Topology => Ok(MaelstromMessageBody {
                node_id: None,
                topology: None,
                id: None,
                msg_type: MessageType::TopologyOk,
                msg_id: self.msg_id,
                in_reply_to: self.msg_id,
                echo: None,
                node_ids: None,
                message: None,
                messages: None,
            }),
        }
    }
}

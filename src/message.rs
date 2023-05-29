use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with;
use uuid::Uuid;

use crate::state::State;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct MaelstromMessage {
    src: String,
    dest: String,
    pub body: MaelstromMessageBody,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct MaelstromMessageBody {
    #[serde(rename = "type")]
    pub msg_type: MessageType,
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
    echo: Option<String>,
    node_id: Option<String>,
    id: Option<String>,
    node_ids: Option<Vec<String>>,
    pub message: Option<usize>,
    messages: Option<Vec<usize>>,
    topology: Option<HashMap<String, Vec<String>>>,
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

impl MaelstromMessage {
    pub fn broadcast(state: &State, message: usize) -> Vec<Self> {
        state
            .topology
            .get(&state.node_id)
            .unwrap()
            .iter()
            .map(|dst_node| MaelstromMessage {
                src: state.node_id.clone(),
                dest: dst_node.clone(),
                body: MaelstromMessageBody::broadcast(message),
            })
            .collect()
    }
    pub fn handle(self, state: &mut State) -> Result<MaelstromMessage, String> {
        let body_result = self.body.handle(state);
        match body_result {
            Ok(body) => Ok(MaelstromMessage {
                src: self.dest,
                dest: self.src,
                body,
            }),
            Err(e) => Err(e),
        }
    }
}

impl MaelstromMessageBody {
    pub fn broadcast(message: usize) -> Self {
        MaelstromMessageBody {
            msg_type: MessageType::Broadcast,
            msg_id: None,
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
    pub fn handle(self, state: &mut State) -> Result<MaelstromMessageBody, String> {
        match self.msg_type {
            MessageType::InitOk
            | MessageType::EchoOk
            | MessageType::TopologyOk
            | MessageType::GenerateOk
            | MessageType::ReadOk
            | MessageType::BroadcastOk => Err(String::from("can't handle response")),
            MessageType::Init => {
                state.node_ids = self.node_ids.unwrap();
                state.node_id = self.node_id.unwrap();
                Ok(MaelstromMessageBody {
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
                })
            }
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
            MessageType::Broadcast => {
                if !state.seen_messages.contains(&self.message.unwrap()) {
                    state.seen_messages.push(self.message.unwrap());
                }
                Ok(MaelstromMessageBody {
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
                })
            }
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
            MessageType::Topology => {
                state.topology = self.topology.unwrap();
                Ok(MaelstromMessageBody {
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
                })
            }
        }
    }
}

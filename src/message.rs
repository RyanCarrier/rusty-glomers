use serde::{Deserialize, Serialize};
use serde_with;
use uuid::Uuid;

use crate::state::State;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct MaelstromMessage {
    src: String,
    dest: String,
    body: MaelstromMessageBody,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct MaelstromMessageBody {
    #[serde(rename = "type")]
    msg_type: MessageType,
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
    echo: Option<String>,
    id: Option<String>,
    node_ids: Option<Vec<String>>,
    message: Option<usize>,
    messages: Option<Vec<usize>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
                Ok(MaelstromMessageBody {
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
                state.seen_messages.push(self.message.unwrap());
                Ok(MaelstromMessageBody {
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

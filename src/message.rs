use serde::{Deserialize, Serialize};
use serde_with;
use uuid::Uuid;

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
}
impl MaelstromMessage {
    pub fn handle(self) -> Result<MaelstromMessage, String> {
        let body_result = self.body.handle();
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
    pub fn handle(self) -> Result<MaelstromMessageBody, String> {
        match self.msg_type {
            MessageType::InitOk | MessageType::EchoOk | MessageType::GenerateOk => {
                Err(String::from("can't handle response"))
            }
            MessageType::Init => Ok(MaelstromMessageBody {
                id: None,
                msg_type: MessageType::InitOk,
                msg_id: self.msg_id,
                in_reply_to: self.msg_id,
                echo: None,
                node_ids: None,
            }),
            MessageType::Echo => Ok(MaelstromMessageBody {
                id: None,
                msg_type: MessageType::EchoOk,
                msg_id: self.msg_id,
                in_reply_to: self.msg_id,
                echo: self.echo,
                node_ids: None,
            }),

            MessageType::Generate => Ok(MaelstromMessageBody {
                id: Some(Uuid::new_v4().to_string()),
                msg_type: MessageType::GenerateOk,
                msg_id: self.msg_id,
                in_reply_to: self.msg_id,
                echo: self.echo,
                node_ids: None,
            }),
        }
    }
}

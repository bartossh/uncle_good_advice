use crate::traits::{Advise, Configur};
use core::str;
use kalosm::language::*;

pub struct Advisor {
    chat: Chat,
}

impl Advisor {
    pub async fn try_new(configurator: impl Configur) -> Result<Self, String> {
        let prompt = configurator.prompt();
        let model = match Llama::new_chat().await {
            Ok(m) => Ok(m),
            Err(e) => Err(e.to_string()),
        }?;
        let chat = Chat::builder(model).with_system_prompt(prompt).build();

        Ok(Self { chat })
    }
}

impl Advise for Advisor {
    async fn advise_about(&mut self, msg: &str) -> Result<String, String> {
        let mut response_stream = self.chat.add_message(msg);
        let mut bytes: Vec<u8> = Vec::new();
        if let Err(e) = response_stream.write_to(&mut bytes).await {
            return Err(e.to_string());
        }

        match str::from_utf8(&bytes) {
            Ok(m) => Ok(m.into()),
            Err(e) => Err(e.to_string()),
        }
    }
}

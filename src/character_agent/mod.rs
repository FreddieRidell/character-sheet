use yew::worker::*;

use serde::{Serialize, Deserialize};

struct Character {
    link: AgentLink<Character>,
    level: u8,
}

pub enum Msg {
    HelloEnum,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Level(u8),
}

impl Transferable for Request { }

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Level(u8),
}

impl Transferable for Response {}

impl Agent for Character {
    type Reach = Context;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    // Create an instance with a link to agent's environment.
    fn create(link: AgentLink<Self>) -> Self {
        Character {
            link,
            level: 1,
        }
    }

    // Handle inner messages (of services of `send_back` callbacks)
    fn update(&mut self, msg: Self::Message) { /* ... */ }

    // Handle incoming messages from components of other agents.
    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::Level(level) => {
                self.level = level;
                self.link.response(who, Response::Level(self.level));
            },
        }
    }
}

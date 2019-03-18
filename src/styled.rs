#[macro_use]
use stdweb::{self,*};
use rand::prelude::*;
use stdweb::unstable::TryInto;

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

pub struct StyleDefinition {
    rules: HashMap<&'static str, String>,
    class: String,
}

impl StyleDefinition {
    pub fn new() -> Self {
        let class: String = js!(
            return new Array(6).fill(null).map( () => "abcdefghijklmnopqrstuvwxyz"[Math.round(Math.random() * 26 )]).join("");
        ).try_into().unwrap();

        stdweb::event_loop();

        StyleDefinition {
            rules: HashMap::new(),
            class,
        }
    }

    pub fn declare<T: Display>(&mut self, key: &'static str, value: T) -> &mut Self {
        let mut rule = String::new();
        rule.push_str(format!(".{} {{ ", &self.class).as_str());

        for (key,val) in self.rules.iter() {
            rule.push_str(format!("{}: {}; ", key, val).as_str());
        }

        let end: &str = "}";
        rule.push_str(end);

        js!{
            console.log(@{&rule});

            document
                .getElementById("generated-styles")
                .sheet
                .insertRule(@{&rule}, 0);
        }

        self.rules.insert(key, format!("{}", value));

        self
    }

    pub fn get_style(&self) -> String {

        format!("{}", self)
    }
}

impl Display for StyleDefinition {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.class)
    }
}

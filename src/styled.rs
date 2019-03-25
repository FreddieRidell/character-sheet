#[macro_use]
use stdweb::{self,*};
use rand::prelude::*;
use stdweb::unstable::TryInto;

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

use diesel_punk::style::Style;

pub fn publish_style(style: &Style) -> String {
    let rule = format!("{:#}", style);

    js!{
        const sheet = document.getElementById("generated-styles").sheet; 
        sheet.insertRule(@{&rule}, sheet.cssRules.length);
    };

    format!("{}", style)
}


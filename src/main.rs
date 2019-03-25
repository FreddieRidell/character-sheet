extern crate stdweb;

use yew::services::storage::Area;
use yew::services::{ConsoleService, StorageService};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

use diesel_punk::style::Style;
use diesel_punk::variable::Variable;

mod styled;
use styled::publish_style;

const color_var: CssVar = CssVarDefinition::<String>::define("color", String::from("black"));
const font_size_var: CssVar = CssVarDefinition::<u8>::define("font-size", 2)
    .format(|x: u8| format!("{}px", x));

const display_style: Style = Style::new("display-style")
    .set("color", color_var)
    .set("font-size", font_size_var)
    .set("margin", "1rem")
    .set("padding", "1rem");

struct App {
    color: CssVar<String>,
    console: ConsoleService,
    font_size: CssVar<u8>,
}

enum Msg {
    Inc,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            color: color_var::new(),
            console: ConsoleService::new(),
            font_size: font_size_var::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Inc => {
                self.font_size.wrapped = self.font_size.wrapped + 1;
                if self.font_size.wrapped > 5 {
                    self.font_size.wrapped = 2
                };

                true
            }
        }
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        let class = format!("{}", display_style);
        let rule_set = String::from("");

        html! {
            <div style=&rule_set_id, >
                <div class=&class, onclick=|_| { Msg::Inc }, > { "Here Is The Text" } </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}

extern crate stdweb;

use yew::services::storage::Area;
use yew::services::{ConsoleService, StorageService};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

use diesel_punk::style::Style;

mod styled; 
use styled::publish_style;

struct App {
    console: ConsoleService,
    header_style: Style,
    i: u8,
}

enum Msg {
Inc
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut header_style = Style::new("header");
        header_style
                .set("background-color", "red")
                .set("margin", "0")
                .set("font-size", "2rem")
                .set("padding", "1rem");

        App {
            console: ConsoleService::new(),
            header_style,
            i: 2,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Inc => {
                self.i = self.i + 1;
                if self.i > 5 { self.i = 2 };
                self.header_style.set("font-size", format!("{}rem", self.i ));

                true
            }
        }
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        let class = publish_style(&self.header_style);

        html! {
            <>
                <header class=&class, onclick=|_| { Msg::Inc }, >{ "foo" }</header>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}

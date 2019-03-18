extern crate stdweb;

use yew::services::storage::Area;
use yew::services::{ConsoleService, StorageService};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod styled;

use styled::StyleDefinition;

struct App {
    console: ConsoleService,
    local_storage: StorageService,
    header_style: StyleDefinition,
}

enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut header_style = StyleDefinition::new();

        header_style
                .declare("background-color", "red")
                .declare("margin", "0")
                .declare("font-size", "2rem")
                .declare("padding", "1rem");

        App {
            console: ConsoleService::new(),
            local_storage: StorageService::new(Area::Local),
            header_style,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        let class = format!("{}", &self.header_style);

        html! {
            <>
                <header class=&class,>{ "foo" }</header>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}

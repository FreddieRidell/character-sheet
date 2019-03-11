use yew::services::storage::Area;
use yew::services::{ConsoleService, StorageService};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

struct Character {
    name: String,
}

impl Character {
    fn new() -> Character {
        Character {
            name: String::from("Mathius Splurtle")
        }
    }
}

struct App {
    character: Character,
    console: ConsoleService,
    local_storage: StorageService,
}

enum Msg {
    DoIt,
    ChangeName(yew::html::ChangeData),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            character: Character::new(),
            console: ConsoleService::new(),
            local_storage: StorageService::new(Area::Local),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                self.console.log("hello dad");
                // Update your model on events
                true
            },

            Msg::ChangeName(html::ChangeData::Value(name)) => {
                self.console.log(name.as_str());
                self.character.name = name;

                true
            },
            Msg::ChangeName(_) => false,
        }
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <header>{ &self.character.name }</header>
                <input onchange=|value| Msg::ChangeName(value), />
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}

use yew::{html, Component, ComponentLink, Html, ShouldRender};

struct App {
    clicked: bool,
}

enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App { clicked: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        let text = if self.clicked {
            "Clicked!"
        } else {
            "Click me!"
        };
        html! {
            <button onclick=|_| Msg::Click>{text}</button>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}

use yew::prelude::*;

//messages the component can receive
enum Msg {
    Addone,
}

struct CounterComponent {
    count: i64,
}
impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Addone => {
                self.count += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class = "container">
                <p>{self.count}</p>
                <button onclick={link.callback(|_| Msg::Addone)}>{"+1"}</button>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<CounterComponent>::new().render();
}

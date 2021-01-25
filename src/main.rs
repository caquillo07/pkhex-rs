use yew::prelude::*;
use pkhexcore::*;

#[derive(Debug)]
pub struct Model {
    pokemon: pkm::pk8::PK8,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            pokemon: pkm::pk8::PK8::from(include_bytes!("../data/test/810_-_Grookey_-_AD1E8DBD44F8.pk8"))
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main>
                <img class="logo" src="http://static.yew.rs/logo.svg" alt="Yew logo" />
                <h1>{ self.pokemon.nickname.as_str() }</h1>
                <span class="subtitle">{ format!("Level: {}", self.pokemon.stat_level) }</span>
                <span class="subtitle">{ format!("OT: {}", self.pokemon.ot_name) }</span>
            </main>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

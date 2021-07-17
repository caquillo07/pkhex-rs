#![recursion_limit = "512"]

mod bindings;

use std::convert::TryFrom;

use pkhexcore::*;
use pkm::pk8;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::reader::ReaderTask;
use yew::services::ReaderService;

#[derive(Debug)]
pub struct LayoutComponent {
    pokemon: Option<pk8::PK8>,
    link: ComponentLink<Self>,
    reader: ReaderService,
    tasks: Vec<ReaderTask>,
}

pub enum Msg {
    FileLoaded(Vec<u8>),
    ClearFile,
    OpenPK8File,
}

impl Component for LayoutComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        LayoutComponent {
            pokemon: None,
            link,
            reader: ReaderService {},
            tasks: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ClearFile => {
                log::info!("hello!!");
            }
            Msg::FileLoaded(file_data) => {
                // todo(hector) - handle errors
                // let c: [u8; 344] = <[u8; 344]>::try_from(file_data.content).unwrap();
                self.pokemon = Some(pk8::PK8::from(
                    &<[u8; 344]>::try_from(file_data.clone()).unwrap(),
                ));
            }
            Msg::OpenPK8File => {
                let callback = self.link.callback(Msg::FileLoaded);
                bindings::open_pk8_file(Closure::once_into_js(move |payload: Vec<u8>| {
                    callback.emit(payload)
                }));
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        log::info!("change func:");
        false
    }

    fn view(&self) -> Html {
        html! {
            <main>
                {
                    if let Some(pokemon) = &self.pokemon {
                        html! {
                            <>
                                <h1>{ pokemon.nickname.as_str() }</h1>
                                <span class="subtitle">{ format!("Level: {}", pokemon.stat_level) }</span>
                                <span class="subtitle">{ format!("OT: {}", pokemon.ot_name) }</span>
                            </>
                        }
                    } else {
                        html! {<></>}
                    }
                }


                <br/>
                <button onclick=self.link.callback(|_| Msg::OpenPK8File) >
                    { "Open File" }
                </button>
            </main>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<LayoutComponent>();
}

#![recursion_limit = "256"]

use std::convert::TryFrom;

use pkhexcore::*;
use pkm::pk8;
use yew::{events::ChangeData, services::ReaderService};
use yew::{prelude::*, services::reader::FileData};
use yew::{services::reader::ReaderTask, web_sys::File};

#[derive(Debug)]
pub struct Model {
    pokemon: Option<pk8::PK8>,
    link: ComponentLink<Self>,
    reader: ReaderService,
    tasks: Vec<ReaderTask>,
}

pub enum Msg {
    FileLoaded(FileData),
    SelectFile(Option<File>),
    ClearFile,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            pokemon: None,
            link,
            reader: ReaderService::new(),
            tasks: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SelectFile(file) => {
                log::info!("file: {:?}", file);
                if let Some(f) = file {
                    log::info!("file: {:?}", f.size());
                    let mut reader = ReaderService::new();
                    let callback = self.link.callback(Msg::FileLoaded);
                    //     todo(hector) - Handle this more gracefully, do not
                    //     ignore the error
                    let task = reader.read_file(f, callback).unwrap();
                    self.tasks.push(task);
                }
            }
            Msg::ClearFile => {}
            Msg::FileLoaded(file_data) => {
                log::info!("fileData: {:?}", file_data);
                // todo(hector) - handle errors
                // let c: [u8; 344] = <[u8; 344]>::try_from(file_data.content).unwrap();
                self.pokemon = Some(pk8::PK8::from(
                    &<[u8; 344]>::try_from(file_data.content.clone()).unwrap(),
                ));
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
                <img class="logo" src="http://static.yew.rs/logo.svg" alt="Yew logo" />
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
                <input type="file" id="file-input" accept=".pk8" onchange=self.link.callback(|cd: ChangeData| {
                    match cd {
                        ChangeData::Files(file_list) => {
                            log::info!("File list: {:?}", file_list.get(0));
                            let file = file_list.get(0);
                            Msg::SelectFile(file_list.get(0))
                        },
                        default => Msg::ClearFile
                    }
                })/>
            </main>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<Model>();
}

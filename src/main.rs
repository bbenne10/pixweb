mod color;
mod components;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use gloo_file::callbacks::FileReader;
use gloo_file::File;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

use crate::color::{find_matches, Color};
use crate::components::ColorTile;

pub enum Msg {
    Loaded(String, String),
    Files(Vec<File>),
}

pub struct App {
    readers: HashMap<String, FileReader>,
    colors: Vec<Color>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            colors: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file_name, data) => {
                self.readers.remove(&file_name);
                self.colors = find_matches(&data);
                true
            }
            Msg::Files(files) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let task = {
                        let file_name = file_name.clone();
                        let link = ctx.link().clone();

                        gloo_file::callbacks::read_as_text(&file, move |res| {
                            log::info!("in cb");
                            link.send_message(Msg::Loaded(
                                file_name,
                                res.unwrap_or_else(|e| e.to_string()),
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let tiles = self.colors.iter().map(|color| {
            html! {
                <ColorTile color={color.to_string()} />
            }
        });
        html! {
            <div>
                <div>
                    <input type="file" multiple=true onchange={ctx.link().callback(move |e: Event| {
                        let mut result = Vec::new();
                        let input: HtmlInputElement = e.target_unchecked_into();

                        if let Some(files) = input.files() {
                            let files = js_sys::try_iter(&files)
                                .unwrap()
                                .unwrap()
                                .map(|v| web_sys::File::from(v.unwrap()))
                                .map(File::from);
                            result.extend(files);
                        }
                        Msg::Files(result)
                    })}
                    />
                </div>
                <div>
                    { for tiles }
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

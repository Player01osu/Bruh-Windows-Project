use crate::routes::GalleryRoute;
use std::rc::Rc;

use yew::{html, Component, Context, Html};
use yew_router::prelude::*;

use yew_router::scope_ext::HistoryHandle;

#[derive(Clone, PartialEq)]
pub struct SortStruct {
    link: GalleryRoute,
    text: Rc<String>,
}

pub struct SortButtons {
    _listener: HistoryHandle,
    sort_current: String,
    sort_current_display: String,
    sort_one: SortStruct,
    sort_two: SortStruct,
}

impl Default for SortStruct {
    fn default() -> Self {
        Self {
            link: GalleryRoute::New,
            text: Rc::new("New".to_string()),
        }
    }
}

pub enum SortButtonsMessage {
    CreateButtons,
}

impl SortButtons {
    fn populate_buttons(&mut self, lookup_num: usize) {
        let lookup_buttons = [
            (Rc::new("New".to_string()), GalleryRoute::New),
            (Rc::new("Top".to_string()), GalleryRoute::Top),
            (Rc::new("Views".to_string()), GalleryRoute::Views),
        ];

        let (text, _) = lookup_buttons.get(lookup_num).unwrap();
        self.sort_current_display = text.to_string();
        let lookup_buttons = lookup_buttons.clone();
        let (text, link) = lookup_buttons.get((lookup_num + 1) % 3).unwrap();

        self.sort_one.link = link.to_owned();
        self.sort_one.text = text.clone();

        let lookup_buttons = lookup_buttons.clone();
        let (text, link) = lookup_buttons.get((lookup_num + 2) % 3).unwrap();

        self.sort_two.link = link.to_owned();
        self.sort_two.text = text.clone();
    }
}

// FIXME: This is bad
impl Component for SortButtons {
    type Properties = ();
    type Message = SortButtonsMessage;

    fn create(ctx: &Context<Self>) -> Self {
        let listener = ctx
            .link()
            .add_history_listener(ctx.link().callback(|_| SortButtonsMessage::CreateButtons))
            .unwrap();

        Self {
            _listener: listener,
            sort_current: String::default(),
            sort_current_display: String::from("New"),
            sort_one: Default::default(),
            sort_two: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SortButtonsMessage::CreateButtons => {
                self.sort_current = ctx
                    .link()
                    .location()
                    .unwrap()
                    .pathname()
                    .split_once("gallery")
                    .unwrap_or(("", ""))
                    .1
                    .to_string();
                match self.sort_current.as_str() {
                    "/new" => {
                        self.populate_buttons(0 as usize);
                    }
                    "/top" => {
                        self.populate_buttons(1 as usize);
                    }
                    "/views" => {
                        self.populate_buttons(2 as usize);
                    }
                    _ => {
                        self.populate_buttons(0 as usize);
                    }
                }
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let current_sort_display = self.sort_current_display.clone();
        let sort_one = self.sort_one.clone();
        let sort_two = self.sort_two.clone();

        html! {
            <>
                <div class="sort-buttons">
                    <button class="dropbtn">{current_sort_display}</button>
                    <div class="sort-button-content">
                        <Link<GalleryRoute> to={sort_one.link} classes="link">{ sort_one.text }</Link<GalleryRoute>>
                        <Link<GalleryRoute> to={sort_two.link} classes="link">{ sort_two.text }</Link<GalleryRoute>>
                    </div>
                </div>
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        self.sort_current = ctx
            .link()
            .location()
            .unwrap()
            .pathname()
            .split_once("gallery")
            .unwrap()
            .1
            .to_string();
        match self.sort_one.eq(&self.sort_two) {
            true => ctx.link().send_message(SortButtonsMessage::CreateButtons),
            false => (),
        }
    }
}

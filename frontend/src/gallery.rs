use crate::components::posts::PostQuery;
use crate::components::sortbuttons::SortButtons;

use super::components::{container::Container, posts::Posts, template::Template};
use gloo_utils::document;
use web_sys::WheelEvent;
use yew::{html, html::Scope, Callback, Component, Context, Html, NodeRef};
use yew_router::{scope_ext::{HistoryHandle, RouterScopeExt}, prelude::Location};

pub struct Gallery {
    _history_handle: HistoryHandle,
    query: PostQuery,
    page_number: u16,
    document_height: f64,
    wheel_position: f64,
    scroll_bottom_buffer: u16,
    posts: Html,
    node_ref: NodeRef,
}

pub enum GalleryMsg {
    LoadMore,
    Reload,
    None,
}

impl Gallery {
    pub fn show_posts(&mut self) {
        let node_ref = self.node_ref.clone();
        let page_number = self.page_number;
        let query = self.query.clone();

        self.posts = html! {
            <>
                <SortButtons query={query.clone()}/>
                <Posts
                    document_height={self.document_height}
                    wheel_position={self.wheel_position}
                    {query}
                    {page_number}
                    gallery_node_ref={node_ref}
                />
            </>
        };
    }
}

impl Component for Gallery {
    type Properties = ();
    type Message = GalleryMsg;

    fn create(ctx: &Context<Self>) -> Self {
        let history_listener = ctx
            .link()
            .add_history_listener(ctx.link().callback(move |_| GalleryMsg::Reload))
            .unwrap();

        let gallery = Self {
            _history_handle: history_listener,
            query: PostQuery::default(),
            document_height: 0.0,
            wheel_position: 0.0,
            page_number: 1,
            scroll_bottom_buffer: 1,
            posts: Html::default(),
            node_ref: NodeRef::default(),
        };

        ctx.link().send_message(GalleryMsg::Reload);
        gallery
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GalleryMsg::LoadMore => {
                match self.scroll_bottom_buffer == 0 {
                    true => {
                        self.page_number += 1;
                        self.show_posts();
                        self.scroll_bottom_buffer = 40;
                    }
                    false => {
                            self.scroll_bottom_buffer -= 1;
                        }
                }

                true
            }

            GalleryMsg::Reload => {
                self.query = ctx.link().location().unwrap().query::<PostQuery>().unwrap();
                self.page_number = 1;
                self.show_posts();
                true
            }
            GalleryMsg::None => false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onwheel = ctx.link().callback(|wheel_event: WheelEvent| {
            // FIXME kinda inconsistent
            let scroll_y = wheel_event.view().unwrap().scroll_y().unwrap();
            let page_height = document()
                .get_element_by_id("loadOnBottom")
                .expect("Element id not found")
                .scroll_height();

            if scroll_y / page_height as f64 > 0.5 {
                //self.page_number += 1;
                GalleryMsg::LoadMore
            } else {
                GalleryMsg::None
            }
                //self.document_height = document_height / 1.58;
                //self.wheel_position = wheel_position * 1.5;
        });

        let node_ref = self.node_ref.clone();
        let show_posts = self.posts.clone();

        html! {
            <>
                <script type="module" src="https://unpkg.com/ionicons@5.5.2/dist/ionicons/ionicons.esm.js"></script>
                <script nomodule=true src="https://unpkg.com/ionicons@5.5.2/dist/ionicons/ionicons.js"></script>
                <Template>
                    <div id="loadOnBottom" ref={ node_ref }{ onwheel }>
                        <Container/>
                        <center>
                            { show_posts }
                        </center>
                    </div>
                </Template>
            </>
        }
    }
}

use crate::components::{
    fileupload::FileUpload,
    posts::{PostQuery, Posts},
    sortbuttons::SortButtons,
};

use gloo_utils::document;
use web_sys::WheelEvent;
use yew::{html, Component, Context, Html, NodeRef};
use yew_router::{
    prelude::Location,
    scope_ext::{HistoryHandle, RouterScopeExt},
};

pub struct Gallery {
    _history_handle: HistoryHandle,
    query: PostQuery,
    page_number: u16,
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
                <Posts
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
            GalleryMsg::None => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onwheel = ctx.link().callback(|wheel_event: WheelEvent| {
            // FIXME kinda inconsistent
            let scroll_y = wheel_event.view().unwrap().scroll_y().unwrap();
            let page_height = document()
                .get_element_by_id("posts")
                .expect("Element id not found")
                .scroll_height();

            if scroll_y / page_height as f64 > 0.5 {
                GalleryMsg::LoadMore
            } else {
                GalleryMsg::None
            }
        });

        let node_ref = self.node_ref.clone();
        let show_posts = self.posts.clone();
        let query = self.query.clone();

        html! {
            <>
                <SortButtons query={query.clone()}/>
                <FileUpload/>
                <div id="posts" ref={ node_ref }{ onwheel }>
                    { show_posts }
                </div>
            </>
        }
    }
}

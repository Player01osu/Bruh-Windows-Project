use super::components::container::Container;
use super::components::header::Header;
use super::components::posts::Posts;
use gloo_utils::document;
use web_sys::WheelEvent;
use yew::{html, Component, Context, Html, NodeRef, Properties};

pub struct Gallery {
    document_height: f64,
    wheel_position: f64,
    node_ref: NodeRef,
}

#[derive(Properties, PartialEq)]
pub struct GalleryProps {
    pub sort: String,
}

pub enum LoadOnBottom {
    LoadMore(f64, f64),
}

impl Component for Gallery {
    type Properties = GalleryProps;
    type Message = LoadOnBottom;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            document_height: 0.0,
            wheel_position: 0.0,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoadOnBottom::LoadMore(document_height, wheel_position) => {
                self.document_height = document_height / 1.58;
                self.wheel_position = wheel_position * 1.5;
                true
            }
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
            LoadOnBottom::LoadMore(page_height.into(), scroll_y)
        });
        let show_posts = html! {
            <Posts sort={ctx.props().sort.clone()}
                document_height={self.document_height}
                wheel_position={self.wheel_position}
                gallery_node_ref={self.node_ref.clone()}/>
        };

        html! {
            <>
                <script type="module" src="https://unpkg.com/ionicons@5.5.2/dist/ionicons/ionicons.esm.js"></script>
                <script nomodule=true src="https://unpkg.com/ionicons@5.5.2/dist/ionicons/ionicons.js"></script>
                <body style="background-color: black;">
                    <Header/>
                    <div id="loadOnBottom" ref={ self.node_ref.clone() }{ onwheel }>
                        <Container/>
                        { show_posts }
                    </div>
                </body>
            </>
        }
    }
}

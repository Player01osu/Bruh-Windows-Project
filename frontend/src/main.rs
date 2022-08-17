mod components;
mod pages;
mod routes;
mod session;

use crate::session::Session;
use components::{container::Container, template::Template};
use gloo_utils::window;
use pages::gallery::GalleryMsg;
use routes::{switch, Route};
use yew::{html, Component, Context, Html, ContextProvider};
use yew_router::{BrowserRouter, Switch};

pub struct App {
    session: Session,
}

pub struct AppMsg(Session);

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Session::init(&ctx.link());
        Self {
            session: Default::default()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let local_storage = window()
            .local_storage()
            .unwrap()
            .expect("Failed to get local storage");

        local_storage
            .set_item("public", &msg.0.user_pub)
            .expect("Failed to set public id");
        if let Some(private) = &msg.0.user_priv {
            local_storage
                .set_item("private", private)
                .expect("Failed to set private id")
        };

        self.session.update_state(msg.0);
        if let Some(callback) = &self.session.gallery_callback {
            callback.emit(GalleryMsg::Reload);
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let session = self.session.clone();
        html! {
            <>
                <BrowserRouter>
                    <Container/>
                    <Template>
                        <ContextProvider<Session> context={session}>
                            <Switch<Route> render={Switch::render(switch)} />
                        </ContextProvider<Session>>
                    </Template>
                </BrowserRouter>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}

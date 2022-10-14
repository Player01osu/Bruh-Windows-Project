mod components;
mod pages;
mod routes;
mod session;

use crate::session::Session;
use components::{container::Container, template::Template};
use gloo_utils::window;
use pages::gallery::GalleryMsg;
use routes::{switch, Route};
use session::ChangeState;
use yew::{html, Component, Context, ContextProvider, Html};
use yew_router::{BrowserRouter, Switch};

pub struct App {
    session: Session,
}

pub enum AppMsg {
    LoadUser(Session),
    UpdateSession(ChangeState),
    None,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Session::init(&ctx.link());
        Self {
            session: Session {
                app_message: ctx.link().callback(|m| m),
                ..Default::default()
            },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::LoadUser(user) => {
                let local_storage = window()
                    .local_storage()
                    .unwrap()
                    .expect("Failed to get local storage");

                local_storage
                    .set_item("public", &user.user_pub)
                    .expect("Failed to set public id");
                if let Some(private) = &user.user_priv {
                    local_storage
                        .set_item("private", private)
                        .expect("Failed to set private id")
                };

                self.session.update_state(user);
                if let Some(callback) = &self.session.gallery_callback {
                    callback.emit(GalleryMsg::Reload);
                }
                true
            },
            AppMsg::UpdateSession(state_change) => {
                self.session.change_state(state_change);
                true
            },
            AppMsg::None => false,
        }
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

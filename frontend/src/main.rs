mod components;

use yew::{html, Component, Context, Html};

use components::posts::Posts;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        html! {
            <>
                <div class={ "header-all" }>
                    <div class={ "header" }>
                        <h1>{ "Wholesome Yuri" }</h1>
                    </div>
                </div>

                <div class={"container"}>

                    <div class="navall">
                            <div class="nav">
                                    <form action="" class="search-bar">
                                            <input type="text" class="search" placeholder="search tag or somth" name="q"/>
                                    </form>
                                    <div class="nav-img">
                                            <div>
                                                    <img class="imge" src="assets/img/blah.jpg" alt="nav-img"/>
                                            </div>
                                    </div>
                                <center>
                                        <div class="links">
                                                <div class="indiv">
                                                        <div>
                                                                <a href="layout2.html"
                                                                    class="link"
                                                                    style="text-decoration: none;">{"LAYOUT2"}</a>
                                                        </div>
                                                </div>
                                                <div class="indiv">
                                                        <div>
                                                                <a href="tags.html"
                                                                    class="link"
                                                                    style="text-decoration: none;">{"TAGS"}</a>
                                                        </div>
                                                </div>
                                                <div class="indiv">
                                                        <div>
                                                                <a href="layout2.html"
                                                                    class="link"
                                                                    style="text-decoration: none;">{"ABOUT"}</a>
                                                        </div>
                                                </div>
                                                <div class="indiv">
                                                        <div>
                                                                <a href="about.html"
                                                                    class="link"
                                                                    style="text-decoration: none;">{"SAMPLE"}</a>
                                                        </div>
                                                </div>
                                        </div>

                                </center>
                            </div>
                    </div>
                </div>

                <div class={ "images" }>
                    <Posts/>
                </div>
            </>
        }
    }
}
fn main() {
    println!("hi");
    yew::start_app::<App>();
}

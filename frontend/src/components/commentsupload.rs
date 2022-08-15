use reqwest::Client;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;

pub struct CommentUpload;

pub enum CommentUploadMsg {
    Submit(FormData),
    None,
}

#[derive(PartialEq, Eq, Properties)]
pub struct CommentUploadProps {
    pub post_id: String,
}

impl Component for CommentUpload {
    type Properties = CommentUploadProps;
    type Message = CommentUploadMsg;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CommentUploadMsg::Submit(form) => {
                let comment_id = &ctx.props().post_id;
                let post_url = format!("http://localhost:7878/api/post-comment/{comment_id}");

                let commenter = form.get("op").as_string().unwrap();
                let comment_body = form.get("body").as_string().unwrap();

                let body = format!(
                    r#"{{
                    "commenter": "{commenter}",
                    "body": "{comment_body}"
                }}"#
                );

                ctx.link().send_future(async move {
                    Client::new()
                        .post(post_url)
                        .header("Content-Type", "application/json")
                        .body(body)
                        .send()
                        .await
                        .unwrap();
                    CommentUploadMsg::None
                });
                true
            }
            CommentUploadMsg::None => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="comment-upload" style="position: absolute; left: 600px; top: 300px;">
                    <form id="comment-upload"
                        onsubmit={ctx.link().callback(move |event: web_sys::FocusEvent| {
                            event.prevent_default();
                            let form = event.target_unchecked_into::<HtmlFormElement>();
                            let data = FormData::new_with_form(&form).unwrap();
                            CommentUploadMsg::Submit(data)
                        })}
                    >
                        <label for="op">{"OP:"}</label><br/>
                        <input type="text" id="op" name="op"/><br/>
                        <label for="body">{"Comment:"}</label><br/>
                        <input type="text" id="body" name="body" required=true/><br/>
                        <br/>
                    </form>
                    <input type="submit"
                        form="comment-upload"
                        value="submit"
                    />
                </div>
            </>
        }
    }
}

use crate::components::commentsupload::CommentUpload;
use common::mongodb::structs::CommentSection;

use reqwasm::http::Request;
use yew::{html, Component, Context, Html, Properties};

pub struct Post {
    comments: Html,
}

#[derive(PartialEq, Eq, Properties)]
pub struct PostProps {
    pub post_id: String,
}

pub enum PostMsg {
    ProcessComments(CommentSection),
}

impl Component for Post {
    type Properties = PostProps;
    type Message = PostMsg;

    fn create(ctx: &Context<Self>) -> Self {
        let post_id = ctx.props().post_id.clone();
        ctx.link().send_future(async move {
            let fetched_comments: CommentSection =
                Request::get(format! {"/api/view-posts/{post_id}"}.as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
            PostMsg::ProcessComments(fetched_comments)
        });

        Self { comments: html! {} }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PostMsg::ProcessComments(comment_section) => {
                self.comments = match comment_section.comments {
                    Some(v) => {
                        if !v.is_empty() {
                            v
                        .into_iter()
                        .map(|f| {
                            let body = f.body;
                            let commenter = f.commenter;
                            html! {
                                <div class="user-comments" style="position: absolute; top: 60px;">
                                    <h1>{commenter}</h1>
                                    <h2>{body}</h2>
                                </div>
                            }
                        })
                        .collect::<Html>()
                        } else {
                            html! {
                                <div class="user-comments" style="position: absolute; top: 60px; left: 200px;">
                                    <h1>{"No comments yet"}</h1>
                                </div>
                            }
                        }
                    }
                    None => html! { //TODO: Make not options
                        <div class="user-comments" style="position: absolute; top: 60px; left: 200px;">
                            <h1>{"No comments yet"}</h1>
                        </div>
                    },
                };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let post_id = ctx.props().post_id.clone();
        html! {
            <>
                <CommentUpload {post_id}/>
                {self.comments.clone()}
            </>
        }
    }
}

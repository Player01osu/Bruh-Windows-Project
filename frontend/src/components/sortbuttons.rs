use crate::routes::Route;

use yew::{html, Component, Context, Html, Properties};
use yew_router::prelude::*;

use super::posts::PostQuery;

pub struct SortButtons;

impl SortButtons {
    fn generate_buttons(query: &PostQuery, index: u16) -> PostQuery {
        let offset = match query.sort.as_str() {
            "new" => 0,
            "top" => 1,
            "views" => 2,
            _ => 0,
        };

        let sort = match (offset + index) % 3 {
            0 => String::from("new"),
            1 => String::from("top"),
            2 => String::from("views"),
            _ => String::from("new"),
        };

        let query = query.query.clone();

        PostQuery { sort, query }
    }

    fn generate_sort_buttons(query: PostQuery) -> Html {
        let sort_current = Self::generate_buttons(&query, 0);
        let sort_one = Self::generate_buttons(&query, 1);
        let sort_two = Self::generate_buttons(&query, 2);

        let sort_current_text = &sort_current.sort.to_uppercase();
        let sort_one_text = &sort_one.sort.to_uppercase();
        let sort_two_text = &sort_two.sort.to_uppercase();

        html! {
            <>
                <button class="dropbtn">{sort_current_text}</button>
                <div class="sort-button-content">
                    <Link<Route, PostQuery>
                        to={Route::Gallery}
                        query={Some(sort_one.clone())}
                        classes="link"
                    >
                        { sort_one_text }
                    </Link<Route, PostQuery>>
                    <Link<Route, PostQuery>
                        to={Route::Gallery}
                        query={Some(sort_two.clone())}
                        classes="link"
                    >
                        { sort_two_text }
                    </Link<Route, PostQuery>>
                </div>
            </>
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct SortButtonsProps {
    pub query: PostQuery,
}

impl Component for SortButtons {
    type Properties = SortButtonsProps;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let sort_buttons = Self::generate_sort_buttons(ctx.props().query.clone());

        html! {
            <>
                <div class="sort-buttons">
                    {sort_buttons}
                </div>
            </>
        }
    }
}

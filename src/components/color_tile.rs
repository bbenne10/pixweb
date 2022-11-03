use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use stylist::yew::{styled_component, use_style};

#[derive(Properties, PartialEq)]
pub struct ColorTileProps {
    pub color: AttrValue,
}

#[styled_component(ColorTile)]
pub fn color_tile(props: &ColorTileProps) -> Html {
    let s = use_style!(
        r#"
       background-color: ${bg};
       width: 10rem;
       height: 10rem;
       line-height: 10rem;
       text-align: center;
       display: inline-block;
    "#,
        bg = props.color.to_string()
    );
    html! {
        <div class={s}>{props.color.to_string()}</div>
    }
}

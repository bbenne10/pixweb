use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use stylist::yew::{styled_component, use_style};

#[derive(Properties, PartialEq)]
pub struct ColorTileProps {
    pub bg: AttrValue,
    pub fg: AttrValue,
}

#[styled_component(ColorTile)]
pub fn color_tile(props: &ColorTileProps) -> Html {
    #[rustfmt::skip]
    let s = use_style!(
        r#"
          color: ${fg};
          background-color: ${bg};
          width: 10rem;
          height: 10rem;
          line-height: 10rem;
          text-align: center;
          display: inline-block;
       "#,
        bg = props.fg.to_string(),
        fg = props.fg.to_string()
    );
    html! {
        <div class={s}>{props.bg.to_string()}</div>
    }
}

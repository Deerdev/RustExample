use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    // 必传参数
    pub title: String,
    pub test: Option<String>,
    pub color: Color,
    pub on_load: Callback<String>,
}

#[derive(PartialEq)]
pub enum Color {
    Normal,
    Ok,
    Error,
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Normal => "normal".to_string(),
            Color::Ok => "ok".to_string(),
            Color::Error => "error".to_string(),
        }
    }
}

#[styled_component(MainTitle)]
pub fn man_title(props: &Props) -> Html {
    let stylesheet = style!(
        r#"
        .normal {
            color: white;
        }
        .ok {
            color: green;
        }
        .error {
            color: red;
        }
    "#
    )
    .unwrap();

    props.on_load.emit("I loaded".to_string());

    html! {
        <div class={stylesheet}>
            <h1 class={props.color.to_string()}>{&props.title}</h1>
        </div>
    }
}

#[function_component(MainTitle2)]
pub fn man_title2(props: &Props) -> Html {
    html! {
        <h1>{&props.title}</h1>
    }
}

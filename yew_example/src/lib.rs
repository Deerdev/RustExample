use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;

mod components;
use components::atoms::main_title::{Color, MainTitle};

// css 编译期嵌入
// 在同级目录下引入文件作为 字符串
const STYLE_FILE: &str = include_str!("main.css");

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

// 也是 function component
#[styled_component(App2)]
pub fn app2() -> Html {
    let main_title_load = Callback::from(|message: String| {
        log!(message);
    });

    let stylesheet = style!(
        r#"
            h1 {
                color: orange;
            }
            p {
                color: white;
            }
            background-color: black;
        "#
    )
    .unwrap();

    let new_stylesheet = Style::new(STYLE_FILE).unwrap();

    html! {
        <>
            <div class={new_stylesheet}>
                <h1>{"Hello, world!"}</h1>
                <p>{"Hello, world!"}</p>
                // inline
                <p class={css!("color: red; font-size: 25px;")}>{"Hello, world!"}</p>
            </div>
            <div class={stylesheet}>
                // MainTitle 里面的样式依然会被这里的 css 影响
                <MainTitle title={"My New Title"} color={Color::Error} on_load={main_title_load} />
                <h1>{"Hello, world!"}</h1>
                <p>{"Hello, world!"}</p>
                // inline
                <p class={css!("color: red; font-size: 25px;")}>{"Hello, world!"}</p>
            </div>
        </>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let name = "Brooks";
    let my_object = MyObject {
        username: name.to_string(),
        favorite_language: "Rust".to_string(),
    };
    // 只能打印基础类型，不能打印 object
    log!("Hello, ", name);
    // the trait bound `JsValue: From<MyObject>` is not satisfied
    // log!(my_object);

    // 通过 json 序列化
    // .unwrap() 处理报错
    log!(serde_json::to_string_pretty(&my_object).unwrap());
    let class = "my_title";
    let message = Some("I am message");

    let tasks = vec![
        "Learn Rust",
        "Learn Yew",
        "Learn gloo",
        "Learn serde",
        "Learn json",
    ];
    html! {
        <div>
            <>
                <h1 class={class}>{"Hello, world!"}</h1>
            </>
            if class == "my_title" {
                <p>{"This is a paragraph."}</p>
            } else {
                <p>{"This is another paragraph."}</p>
            }
            if let Some(message) = message {
                <p>{message}</p>
            } else {
                <p>{"This is a default message."}</p>
            }
            <ul>
                { tasks.iter().map(|task| html! {<li>{task}</li>}).collect::<Html>() }
                // for 关键字，非 rust 语法
                {for tasks.iter().map(|task| html! {<li>{task}</li>})}
                {list_to_task(tasks)}
            </ul>
        </div>
    }
}

fn list_to_task(list: Vec<&str>) -> Vec<Html> {
    return list.iter().map(|task| html! {<li>{task}</li>}).collect();
}

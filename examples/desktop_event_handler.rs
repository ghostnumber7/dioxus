use dioxus::prelude::*;
use dioxus_desktop::tao::{event::Event, menu::*};
use futures_channel::mpsc::*;
use futures_util::StreamExt;
use std::boxed::Box;
use std::cell::Cell;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
enum BackgroundColorChangeEvent {
    Background(String),
    Font(String),
}

struct AppProps {
    receiver: Cell<Option<Box<UnboundedReceiver<BackgroundColorChangeEvent>>>>,
    menu_colors_mapping: Vec<String>,
}

const STYLES: &str = r#"
<style>
    .container {
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        display: flex;
        justify-content: center;
        align-items: center;
    }
    .center {
        display: inline-block;
        margin: auto;
        background: white;
        padding: 70px 30px;
        border: 1px solid black;
    }
    .center ul {
        width: 40%;
        float:left;
    }
</style>
"#;

fn main() {
    let mut root_menu = MenuBar::new();
    let mut menu_bar_menu = MenuBar::new();

    let menu_colors_mapping = vec![
        "white".to_string(),
        "black".to_string(),
        "red".to_string(),
        "green".to_string(),
        "blue".to_string(),
        "yellow".to_string(),
        "cyan".to_string(),
        "magenta".to_string(),
        "orange".to_string(),
        "pink".to_string(),
    ];

    let mut background_menu_bar = MenuBar::new();
    let mut font_menu_bar = MenuBar::new();

    let background_menu_colors: HashMap<MenuId, String> = menu_colors_mapping
        .iter()
        .enumerate()
        .map(|(index, color)| {
            (
                background_menu_bar
                    .add_item(
                        MenuItemAttributes::new(&format!("{} background", color))
                            .with_accelerators(&format!("Ctrl+{}", index).parse().unwrap()),
                    )
                    .id(),
                color.to_string(),
            )
        })
        .collect();

    let font_menu_colors: HashMap<MenuId, String> = menu_colors_mapping
        .iter()
        .enumerate()
        .map(|(index, color)| {
            (
                font_menu_bar
                    .add_item(
                        MenuItemAttributes::new(&format!("{} font", color))
                            .with_accelerators(&format!("Alt+{}", index).parse().unwrap()),
                    )
                    .id(),
                color.to_string(),
            )
        })
        .collect();

    menu_bar_menu.add_native_item(MenuItem::Quit);

    root_menu.add_submenu("Menu", true, menu_bar_menu);
    root_menu.add_submenu("Background Color", true, background_menu_bar);
    root_menu.add_submenu("Font Color", true, font_menu_bar);

    let (sender, receiver) = unbounded::<BackgroundColorChangeEvent>();

    dioxus_desktop::launch_with_props(
        app,
        AppProps {
            receiver: std::cell::Cell::new(Some(Box::new(receiver))),
            menu_colors_mapping: menu_colors_mapping.clone(),
        },
        |cfg| {
            cfg.with_custom_head(STYLES.to_string())
                .with_window(|w| w.with_title("Menu Event Demo").with_menu(root_menu))
                .with_event_handler(move |event, _webview| match event {
                    Event::MenuEvent {
                        menu_id,
                        origin: MenuType::MenuBar,
                        ..
                    } => {
                        if let Some(color) = background_menu_colors.get(menu_id) {
                            sender
                                .unbounded_send(BackgroundColorChangeEvent::Background(
                                    color.to_string(),
                                ))
                                .unwrap();
                        } else if let Some(color) = font_menu_colors.get(menu_id) {
                            sender
                                .unbounded_send(BackgroundColorChangeEvent::Font(color.to_string()))
                                .unwrap();
                        }
                    }
                    _ => {}
                })
        },
    );
}

fn app(cx: Scope<AppProps>) -> Element {
    let background = use_state(&cx, || "white".to_string());
    let font = use_state(&cx, || "black".to_string());

    use_future(&cx, (), move |_| {
        let receiver = cx.props.receiver.take();
        let background = background.clone();
        let font = font.clone();
        async move {
            if let Some(mut receiver) = receiver {
                loop {
                    tokio::time::sleep(Duration::from_secs(1)).await;

                    while let Some(msg) = receiver.next().await {
                        match msg {
                            BackgroundColorChangeEvent::Background(color) => {
                                background.set(color);
                            }
                            BackgroundColorChangeEvent::Font(color) => {
                                font.set(color);
                            }
                        }
                    }
                    println!("**** not waiting anymore {}", "");
                }
            }
        }
    });

    cx.render(rsx! (
        div {
            class: "container",
            style: "background: {background}; color: {font};",

            div {
                class: "center",

                h3 { "Click on the menu bar to change the background color" }
                h4 { "Or use key bindings" }
                div {
                    ul {
                        li {
                            "Background color:"
                        }
                        cx.props.menu_colors_mapping.iter().enumerate().map(|(index, color)| {
                            rsx! {
                                li {
                                    "Ctrl+{index} = {color}"
                                }
                            }
                        })
                    }
                    ul {
                        li {
                            "Font color:"
                        }
                        cx.props.menu_colors_mapping.iter().enumerate().map(|(index, color)| {
                            rsx! {
                                li {
                                    "Alt+{index} = {color}"
                                }
                            }
                        })
                    }
                }
            }
        }
    ))
}

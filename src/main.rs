#![allow(non_snake_case)]

use dioxus::prelude::*;
use std::{collections::HashSet, fmt::Display, rc::Rc, sync::Arc};
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Select {
            options: vec![
                ("Rust", "Rust"),
                ("Java", "Java"),
                ("Python", "Python"),
                ("Html", "Html"),
            ].into_iter().collect(),
            multiple: true
        }
    }
}

#[component]
fn Tag(onclick: Option<EventHandler<MouseEvent>>, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-auto flex-row-reverse",
            div {
                onclick: move |evt| {
                    if let Some(onclick) = onclick {
                        onclick.call(evt);
                    }
                },
                { children }
                svg {
                    "fill": "none",
                    "stroke-linecap": "round",
                    "stroke": "currentColor",
                    width: "100%",
                    height: "100%",
                    "stroke-width": "2",
                    "stroke-linejoin": "round",
                    "xmlns": "http://www.w3.org/2000/svg",
                    "viewBox": "0 0 24 24",
                    class: "feather inline-block feather-x cursor-pointer hover:text-indigo-400 rounded-full w-4 h-4 ml-2",
                    line {
                        "y2": "18",
                        "x1": "18",
                        "y1": "6",
                        "x2": "6"
                    }
                    line {
                        "x1": "6",
                        "y1": "6",
                        "y2": "18",
                        "x2": "18"
                    }
                }
            }
        }
    }
}

#[component]
fn ArrowIcon() -> Element {
    rsx! {
        div { class: "text-gray-300 w-8 py-1 pl-2 pr-1 border-l flex items-center border-gray-200 svelte-1l8159u",
            button { class: "cursor-pointer w-6 h-6 text-gray-600 outline-none focus:outline-none transform -scale-y-100",
                svg {
                    "stroke-linejoin": "round",
                    "fill": "none",
                    width: "100%",
                    "xmlns": "http://www.w3.org/2000/svg",
                    "stroke-width": "2",
                    "stroke-linecap": "round",
                    "stroke": "currentColor",
                    "viewBox": "0 0 24 24",
                    height: "100%",
                    class: "feather feather-chevron-up w-4 h-4",
                    polyline { "points": "18 15 12 9 6 15" }
                }
            }
        }
    }
}

#[component]
fn Item(on: bool, children: Element, onclick: Option<EventHandler<MouseEvent>>) -> Element {
    rsx! {
        div { class: "cursor-pointer w-full border-gray-100 rounded-b hover:bg-indigo-100",
            onclick: move |evt| {
                if let Some(handler) = onclick {
                    handler.call(evt);
                }
            },
            div { class: "flex w-full items-center p-2 pl-2  border-l-2 relative",
                class: if on { "border-indigo-600" } else {"border-transparent"},
                div { class: "w-full items-center flex",
                    div { class: "mx-2 leading-6", { children } }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub(crate) struct Props<K: Clone + Display + PartialEq + 'static, V: Clone + PartialEq + 'static> {
    onchange: Option<EventHandler<(usize, Arc<[(K, V)]>)>>,
    options: Arc<[(K, V)]>,
    #[props(default = false)]
    open: bool,
    #[props(extends = select)]
    attributes: Vec<Attribute>,
}

#[component]
#[must_use]
pub(crate) fn Select<
    K: Clone + Display + PartialEq + IntoAttributeValue + 'static,
    V: Clone + PartialEq + IntoAttributeValue + 'static,
>(
    props: Props<K, V>,
) -> Element {
    let mut state: Signal<HashSet<usize>> = use_signal(|| HashSet::new());
    let mut open: Signal<bool> = use_signal(|| props.open);

    let mut input: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    rsx! {
        div { class: "w-full md:w-1/2 flex flex-col items-center h-64 mx-auto",
            div { class: "w-full px-4",
                div { class: "flex flex-col items-center relative",
                    // render the input field and also any item selected
                    div { class: "w-full svelte-1l8159u",
                        div { class: "my-2 p-1 flex border border-gray-200 bg-white rounded",
                            div { class: "flex flex-auto flex-wrap",
                                for index in state() {
                                    Tag {
                                        onclick: move |_| {
                                            state.write().remove(&index);
                                        },
                                        "{props.options[index].0}"
                                    }
                                }
                                div { class: "flex-1",
                                    input {
                                        onmounted: move |evt: Event<MountedData>| {
                                            *input.write() = Some((*evt).clone());
                                        },
                                        onfocusin: move |_| {
                                            *open.write() = true;
                                            tracing::info!("focusing in input");
                                        },
                                        onblur: move |_| {
                                            *open.write() = false;
                                            tracing::info!("on blur input");
                                        },
                                        class: "bg-transparent p-1 px-2 appearance-none outline-none h-full w-full text-gray-800"
                                    }
                                }
                            }
                            ArrowIcon {}
                        }
                    }

                    // populate the list of options
                    if open() {
                        div {
                            style: "top: 100%; max-height: 300px;",
                            class: "absolute shadow bg-white z-40 w-full lef-0 rounded overflow-y-auto",
                            div { class: "flex flex-col w-full",
                                for (index, (label, _)) in props.options.clone().into_iter().enumerate() {
                                    Item {
                                        on: state.read().contains(&index),
                                        onclick: {
                                            let options = props.options.clone();
                                            move |evt: Event<MouseData>| {
                                                evt.stop_propagation();
                                                tracing::info!("item click");

                                                // manage internal state for selected/unselected
                                                if state.read().contains(&index) {
                                                    state.write().remove(&index);
                                                } else {
                                                    state.write().insert(index);
                                                }

                                                // set input field to focus
                                                if let Some(input) = input() {
                                                    let _ = input.set_focus(true);
                                                }

                                                // dismiss THIS list by assign false to `open`
                                                *open.write() = false;

                                                // handle any external/provided onchange
                                                if let Some(onchange) = props.onchange {
                                                    onchange.call((index, options.clone()));
                                                }
                                            }
                                        },
                                        "{label}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

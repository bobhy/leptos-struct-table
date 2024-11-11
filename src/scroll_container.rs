use leptos::html::*;
use leptos::prelude::*;
use leptos_use::core::ElementMaybeSignal;
use leptos_use::use_document;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

// Make Option<Element> safe for Signal with SendWrapper
// This makes it safe to send to another thread,
// so long as it's returned to this thread and only ever used here.
#[derive(Copy, Clone, Debug)]
pub struct ScrollContainer(Signal<SendWrapper<Option<web_sys::Element>>>);

impl Default for ScrollContainer {
    fn default() -> Self {
        Self(Signal::derive(move || {
            SendWrapper::new(
                use_document()
                    .body()
                    .as_ref()
                    .map(|w| w.unchecked_ref::<web_sys::Element>().clone()),
            )
        }))
    }
}

impl From<web_sys::Element> for ScrollContainer {
    fn from(element: web_sys::Element) -> Self {
        //bugbug -- compiler complains '`*mut u8` cannot be sent between threads safely'
        //          Even though the element is wrapped in a SendWrapper??
        Self(Signal::derive(move || {
            SendWrapper::new(Some(element.clone()))
        }))
    }
}

impl From<Option<web_sys::Element>> for ScrollContainer {
    fn from(element: Option<web_sys::Element>) -> Self {
        //bugbug -- compiler complains '`*mut u8` cannot be sent between threads safely'
        //          Even though the element is wrapped in a SendWrapper??
        Self(Signal::derive(move || SendWrapper::new(element.clone())))
    }
}

impl<T> From<NodeRef<T>> for ScrollContainer
where
    T: ElementType + Clone + 'static,
{
    fn from(node_ref: NodeRef<T>) -> Self {
        Self(Signal::derive(move || {
            node_ref.get().map(|el| {
                let el: &web_sys::Element = &el.into_any();
                el.clone()
            })
        }))
    }
}

impl From<&str> for ScrollContainer {
    fn from(selector: &str) -> Self {
        let selector = selector.to_owned();

        Self(Signal::derive(move || {
            SendWrapper::new(use_document().query_selector(&selector).unwrap_or_default())
        }))
    }
}

impl From<ScrollContainer> for ElementMaybeSignal<web_sys::Element> {
    fn from(scroll_container: ScrollContainer) -> Self {
        scroll_container.0.into()
    }
}

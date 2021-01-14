//! This module contains the `App` struct, which is used to bootstrap
//! a component in an isolated scope.

use crate::html::{Component, ComponentLink, NodeRef, Scope};
use crate::utils::document;
use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::web::{Element, INode, IParentNode};
    } else if #[cfg(feature = "web_sys")] {
        use web_sys::Element;
    }
}

/// An instance of an application.
#[derive(Debug)]
pub struct App<COMP: Component> {
    /// `Scope` holder
    scope: Scope<COMP>,
}

impl<COMP> Default for App<COMP>
where
    COMP: Component,
{
    fn default() -> Self {
        App::new()
    }
}

impl<COMP> App<COMP>
where
    COMP: Component,
    COMP::Properties: Default,
{
    /// The main entry point of a Yew program. It works similarly to the `program`
    /// function in Elm. You should provide an initial model, `update` function
    /// which will update the state of the model and a `view` function which
    /// will render the model to a virtual DOM tree. If you would like to pass props,
    /// use the `mount_with_props` method.
    pub fn mount(self, element: Element) -> ComponentLink<COMP> {
        clear_element(&element);
        self.scope.mount_in_place(
            element,
            NodeRef::default(),
            NodeRef::default(),
            COMP::Properties::default(),
        )
    }

    /// Alias to `mount("body", ...)`.
    pub fn mount_to_body(self) -> ComponentLink<COMP> {
        // Bootstrap the component for `Window` environment only (not for `Worker`)
        let element = document()
            .query_selector("body")
            .expect("can't get body node for rendering")
            .expect("can't unwrap body node");
        self.mount(element)
    }

    /// Alternative to `mount` which replaces the body element with a component which has a body
    /// element at the root of the HTML generated by its `view` method. Use this method when you
    /// need to manipulate the body element. For example, adding/removing app-wide
    /// CSS classes of the body element.
    pub fn mount_as_body(self) -> ComponentLink<COMP> {
        let html_element = document()
            .query_selector("html")
            .expect("can't get html node for rendering")
            .expect("can't unwrap html node");
        let body_element = document()
            .query_selector("body")
            .expect("can't get body node for rendering")
            .expect("can't unwrap body node");
        html_element
            .remove_child(&body_element)
            .expect("can't remove body child");
        self.scope.mount_in_place(
            html_element,
            NodeRef::default(),
            NodeRef::default(),
            COMP::Properties::default(),
        )
    }
}

impl<COMP> App<COMP>
where
    COMP: Component,
{
    /// Creates a new `App` with a component in a context.
    pub fn new() -> Self {
        let scope = Scope::new(None);
        App { scope }
    }

    /// The main entry point of a Yew program which also allows passing properties. It works
    /// similarly to the `program` function in Elm. You should provide an initial model, `update`
    /// function which will update the state of the model and a `view` function which
    /// will render the model to a virtual DOM tree.
    pub fn mount_with_props(
        self,
        element: Element,
        props: COMP::Properties,
    ) -> ComponentLink<COMP> {
        clear_element(&element);
        self.scope
            .mount_in_place(element, NodeRef::default(), NodeRef::default(), props)
    }

    /// Alias to `mount_with_props("body", ...)`.
    pub fn mount_to_body_with_props(self, props: COMP::Properties) -> ComponentLink<COMP> {
        // Bootstrap the component for `Window` environment only (not for `Worker`)
        let element = document()
            .query_selector("body")
            .expect("can't get body node for rendering")
            .expect("can't unwrap body node");
        self.mount_with_props(element, props)
    }

    /// Alternative to `mount_with_props` which replaces the body element with a component which
    /// has a body element at the root of the HTML generated by its `view` method. Use this method
    /// when you need to manipulate the body element. For example, adding/removing app-wide
    /// CSS classes of the body element.
    pub fn mount_as_body_with_props(self, props: COMP::Properties) -> ComponentLink<COMP> {
        let html_element = document()
            .query_selector("html")
            .expect("can't get html node for rendering")
            .expect("can't unwrap html node");
        let body_element = document()
            .query_selector("body")
            .expect("can't get body node for rendering")
            .expect("can't unwrap body node");
        html_element
            .remove_child(&body_element)
            .expect("can't remove body child");
        self.scope
            .mount_in_place(html_element, NodeRef::default(), NodeRef::default(), props)
    }
}

/// Removes anything from the given element.
fn clear_element(element: &Element) {
    while let Some(child) = element.last_child() {
        element.remove_child(&child).expect("can't remove a child");
    }
}

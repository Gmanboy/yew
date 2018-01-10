//! This module contains the implementation of a virtual component `VComp`.

use std::rc::Rc;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::any::TypeId;
use stdweb::web::Element;
use html::{ScopeBuilder, SharedContext, Component, ComponentUpdate, ScopeSender, Callback};

struct Hidden;

/// A virtual component.
pub struct VComp<CTX, COMP: Component<CTX>> {
    type_id: TypeId,
    props: Option<(TypeId, *mut Hidden)>,
    blind_sender: Box<FnMut((TypeId, *mut Hidden))>,
    generator: Box<FnMut(SharedContext<CTX>, Element)>,
    activators: Vec<Rc<RefCell<Option<ScopeSender<CTX, COMP>>>>>,
    _parent: PhantomData<COMP>,
}

impl<CTX: 'static, COMP: Component<CTX>> VComp<CTX, COMP> {
    /// This method prepares a generator to make a new instance of the `Component`.
    pub fn lazy<CHILD: Component<CTX>>() -> (CHILD::Properties, Self) {
        let builder: ScopeBuilder<CTX, CHILD> = ScopeBuilder::new();
        let mut sender = builder.sender();
        let mut builder = Some(builder);
        let generator = move |context, element| {
            let builder = builder.take().expect("tried to mount component twice");
            builder.build(context).mount(element);
        };
        let mut previous_props = None;
        let blind_sender = move |(type_id, raw): (TypeId, *mut Hidden)| {
            if type_id != TypeId::of::<CHILD>() {
                panic!("tried to send properties of other component");
            }
            let props = unsafe {
                let raw: *mut CHILD::Properties = ::std::mem::transmute(raw);
                *Box::from_raw(raw)
            };
            let new_props = Some(props);
            // Ignore update till properties changed
            if previous_props != new_props {
                let props = new_props.as_ref().unwrap().clone();
                sender.send(ComponentUpdate::Properties(props));
                previous_props = new_props;
            }
        };
        let properties = Default::default();
        let comp = VComp {
            type_id: TypeId::of::<CHILD>(),
            props: None,
            blind_sender: Box::new(blind_sender),
            generator: Box::new(generator),
            activators: Vec::new(),
            _parent: PhantomData,
        };
        (properties, comp)
    }

    /// Attach properties associated with the component.
    pub fn set_props<T>(&mut self, props: T) {
        let boxed = Box::into_raw(Box::new(props));
        let data = unsafe { ::std::mem::transmute(boxed) };
        self.props = Some((self.type_id, data));
    }

    pub(crate) fn send_props(&mut self) {
        let props = self.props.take()
            .expect("tried to send same properties twice");
        (self.blind_sender)(props);
    }

    /// This methods gives sender from older node.
    pub(crate) fn grab_sender_of(&mut self, other: Self) {
        assert_eq!(self.type_id, other.type_id);
        // Grab a sender to reuse it later
        self.blind_sender = other.blind_sender;
    }
}

/// Converts property and stores activator to attach sender in the rendering state.
pub trait Transformer<CTX, COMP: Component<CTX>, FROM, TO> {
    fn transform(&mut self, from: FROM) -> TO;
}

impl<CTX, COMP, T> Transformer<CTX, COMP, T, T> for VComp<CTX, COMP>
where
    COMP: Component<CTX>,
{
    fn transform(&mut self, from: T) -> T {
        from
    }
}

impl<'a, CTX, COMP, T> Transformer<CTX, COMP, &'a T, T> for VComp<CTX, COMP>
where
    COMP: Component<CTX>,
    T: Clone,
{
    fn transform(&mut self, from: &'a T) -> T {
        from.clone()
    }
}

impl<'a, CTX, COMP, F, IN> Transformer<CTX, COMP, F, Option<Callback<IN>>> for VComp<CTX, COMP>
where
    CTX: 'static,
    COMP: Component<CTX>,
    F: Fn(IN) -> COMP::Msg + 'static,
{
    fn transform(&mut self, from: F) -> Option<Callback<IN>> {
        let cell = Rc::new(RefCell::new(None));
        self.activators.push(cell.clone());
        let callback = move |arg| {
            let msg = from(arg);
            if let Some(ref mut sender) = *cell.borrow_mut() {
                sender.send(ComponentUpdate::Message(msg));
            } else {
                panic!("unactivated callback, parent component have to activate it");
            }
        };
        Some(callback.into())
    }
}

impl<CTX: 'static, COMP: Component<CTX>> VComp<CTX, COMP> {
    /// This methods mount a virtual component with a generator created with `lazy` call.
    pub fn mount(&mut self, element: &Element, context: SharedContext<CTX>) {
        (self.generator)(context, element.clone());
    }
}

impl<CTX, COMP: Component<CTX>> PartialEq for VComp<CTX, COMP> {
    fn eq(&self, other: &VComp<CTX, COMP>) -> bool {
        self.type_id == other.type_id
    }
}

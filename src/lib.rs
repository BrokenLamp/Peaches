#![feature(unboxed_closures)]
#![feature(fn_traits)]

use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;

#[macro_use]
extern crate peaches_macros;

pub use peaches_macros::component;

pub struct State {
    map: HashMap<i32, Box<dyn Any>>,
    pos: i32,
    is_first_mount: bool,
}

impl State {
    pub fn new(default: Option<HashMap<i32, Box<dyn Any>>>) -> Self {
        State {
            map: match default {
                Some(map) => map,
                None => HashMap::new(),
            },
            pos: 0,
            is_first_mount: true,
        }
    }

    pub fn top(&mut self) {
        self.pos = 0;
        self.is_first_mount = false;
    }

    pub fn next<'a, T>(&mut self, value: Box<T>) -> (&T, StateSetter<T>)
    where
        T: Any + 'a,
    {
        let pos = self.pos;
        self.pos += 1;
        let val: &T = match self.map.get(&pos) {
            Some(val) => val.as_ref().downcast_ref().unwrap(),
            None => {
                self.map.insert(pos, value);
                value.as_ref()
            }
        };
        (val, StateSetter::new(self, pos))
    }
    pub fn set<'a, T>(&mut self, pos: i32, value: Box<T>)
    where
        T: Any + 'a,
    {
        self.map.insert(pos, value);
    }
}

pub struct StateSetter<'a, T> {
    state: &'a State,
    pos: i32,
    phantom_data: PhantomData<T>,
}

impl<'a, T> StateSetter<'a, T> {
    pub fn new(state: &'a State, pos: i32) -> Self {
        StateSetter {
            state: state,
            pos: pos,
            phantom_data: PhantomData,
        }
    }
}

impl<'a, T> FnOnce<(T,)> for StateSetter<'a, T> {
    type Output = ();
    extern "rust-call" fn call_once(self, args: (T,)) {}
}

pub trait Component {
    fn render(&mut self, state: &mut State) -> Option<Box<dyn Component>>;
}

#[macro_export]
macro_rules! use_state {
    ($value:expr) => {
        state.next(Box::new($value))
    };
}

#[macro_export]
macro_rules! com {
    (
        < $name:ident $($prop_name:ident = { $prop_value:expr } )* > {
            $( $child:expr )*
        }
    ) => {
        Some(Box::new({
            let component = $name::new(vec![$($child),*]);
            $( component.$prop_name = Some($prop_value); )*
            component
        }))
    };
    (
        < > {
            $( $child:expr )*
        }
    ) => {
        com!(<PeachesFragment> {
            $( $child:expr )*
        })
    };
}

#[macro_export]
macro_rules! prop {
    (
        $name:ident
    ) => {
        self.$name
    };
}

struct StyleRule {
    props: HashMap<&'static str, Box<dyn Any>>,
}

impl StyleRule {
    pub fn new() -> Self {
        StyleRule {
            props: HashMap::new(),
        }
    }
    pub fn add(self, name: &'static str, value: Box<dyn Any>) -> Self {
        self.props.insert(name, value);
        self
    }
}

#[macro_export]
macro_rules! use_effect {
    (
        $($vars:expr),* => $block:expr
    ) => {};
}

#[macro_export]
macro_rules! make_styles {
    (
        $name:ident ($($prop_name:ident : $prop_type:ty,)* ) => {
            $($selector_name:ident : {
                $($property_name:ident : $property_value:expr,)*
            })*
        }
    ) => {
        struct $name {
            $($selector_name: StyleRule),*,
            none: StyleRule,
        }
        impl $name {
            pub fn new($($prop_name: $prop_type),*) -> Self {
                $name {
                    $($selector_name:
                        StyleRule::new()
                            $(.add("$property_name", Box::new($property_value)))*
                    ),*,
                    none: StyleRule::new(),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! use_styles {
    (
        $struct:ty [ $($prop_value:expr),* ]
    ) => {
        <$struct>::new($($prop_value),*)
    }
}

component!(Div(
    class: StyleRule,
    text: &String
) => {

    None
});

component!(Text(
    value: String,
    class: StyleRule
) => {
    <Div text={props.value?} class={props.class?} />
});

// Stateless component
component!(App() => {
    let styles = use_styles!(App ["#37474F", "#009688"]);

    <Div class={styles.root}>
        <Text value={"Hello World!"} class={styles.hello_world} />
    </Div>
});

// Stateful component
component!(App(
    color: String,
) => {
    let styles = use_styles!(App [props.color?])
    let is_clicked = use_state!(false);

    let text_component = if is_clicked {
        <Text value={"You Clicked!"} />
    } else {
        <Text value={"Click the button"} />
    };

    <Div class={styles.root}>
        {text_component}
        <Button value={"Click Me!"} class={styles.button} on_click={|| set!(is_clicked => true)} />
    </Div>
});

// Custom props
component!(MaterialButton(
    color: String,
    text: String,
    on_click: DomEventCallback,
) => {
    let styles = use_styles!(MaterialButtonStyles [color]);

    <Button class={styles.root} on_click?={props.on_click}>
        <Text value={props.text?} />
    </Button>
});

make_styles!(App(
    bg: &'static str,
    fg: &'static str,
) => {
    root: {
        display: "flex",
        width: "100vw",
        height: "100vh",
        justify_content: "center",
        align_items: "center",
        background_color: bg,
    }
    hello_world: {
        font_size: "32px",
        color: fg,
    }
});

#[test]
fn test() {}

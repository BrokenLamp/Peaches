#![feature(unboxed_closures)]
#![feature(fn_traits)]

use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;

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
    fn render(&mut self, state: State, children: Vec<&mut dyn Component>);
}

#[macro_export]
macro_rules! component {
    (
        $name:ident ($($prop_name:ident : $prop_type:ty),*) => $block:block
    ) => {
        struct $name {
            $( $prop_name: $prop_type );*
        }

        impl Component for $name {
            fn render(&mut self, state: State, children: Vec<&mut dyn Component>) $block
        }
    };
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
            $($child:expr)*
        }
    ) => {
        {
            let component = $name {
                $($prop_name: $prop_value),*
            };
            let children = vec![$($child),*];
            component.render(use_state!(State::new(None)), children);

        }
    }
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
        $name:ident ($($prop_name:ident : $prop_type:ty),*, ) => {
            $($selector_name:ident : {
                $($property_name:ident : $property_value:expr);*;
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
        $struct:ty |> $($prop_value:expr),*
    ) => {
        <$struct>::new($($prop_value),*)
    }
}

component!(App(bob: i32) => {
    let (bananas, set_bananas) = use_state!(42);
    let styles = use_styles!(AppStyles |> true);

    use_effect!(bob, styles => {
        let interval = set_interval!({}, 1000);
        clear_effect! {
            clear_interval!(interval);
        }
    });

    com!(<Div class={styles.root}> {
        com!(<Div> {
            com!(<Text value={format!("Bananas: {}", bananas)}> {})
        })
        com!(<Div> {
            com!(<Text value={"Add banana".into()}> {})
        })
    })
});

component!(Div(class: StyleRule) => {

});

component!(Text(value: String) => {

});
make_styles!(AppStyles(
    is_collapsed: bool,
) => {
    root: {
        border_radius: 5;
    }
    sub: {
        display: if is_collapsed { "none" } else { "block" };
    }
});

#[test]
fn test() {
    let state = State::new(None);
    com!(<App bob={17}> {});
}

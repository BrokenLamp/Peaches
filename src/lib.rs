use std::collections::HashMap;

pub struct State {
    map: HashMap<&'static str, StateUnit>,
}

impl State {
    pub fn new(default: Option<HashMap<&'static str, StateUnit>>) -> Self {
        State {
            map: match default {
                Some(map) => map,
                None => HashMap::new(),
            },
        }
    }
}

pub enum StateUnit {
    Str(String),
    Number(f64),
    Int(i32),
    Bool(bool),
    SubState(State),
}

pub trait Component {
    fn render(&mut self, state: State, children: Vec<&mut dyn Component>);
}

#[macro_export]
macro_rules! component {
    ($name:ident ($($prop_name:ident : $prop_type:ty),*) => $block:block
    ) => {
        struct $name {
            $( $prop_name: $prop_type );*
        }

        impl Component for $name {
            fn render(&mut self, state: State, children: Vec<&mut dyn Component>) $block
        }
    };
}

component!(App(bob: i32) => {
    
});

#[test]
fn test() {
    App {
        bob: 17,
    }.render(State::new(None), Vec::new());
}

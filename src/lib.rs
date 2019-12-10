use std::collections::HashMap;
use std::any::Any;

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

#[macro_export]
macro_rules! com {
    (
        < $name:ident $($prop_name:ident = ( $prop_value:expr ) )* > {
            $($child:expr)*
        }
    ) => {
        {
            let children: Vec<dyn Any> = Vec::new();
            $(children.push($child);)*
            $name {
                $($prop_name: $prop_value),*
            }.render(use_state!(StateUnit::SubState(State::new(None))), children)
        }
    };
    (
        < $name:ident $($prop_name:ident = ( $prop_value:expr ) )* / >
    ) => {
        com!(< $name $($prop_name = ( $prop_value ))* > { })
    }
}


component!(App(bob: i32) => {
    let (bananas, set_bananas) = use_state!(42);

    com!(<Div> {
        com!(<Div bob=(11)> {
            format!("Bananas: {}", bananas)
        })
        com!(<Div onclick=(|_| set_bananas(|b| b + 1))> {
            "Add banana"
        })
    })
});

#[test]
fn test() {
    App {
        bob: 17,
    }.render(State::new(None), Vec::new());
}

// Renders as
/*
<div>
    bob: 17
    bananas: 42
    <div onclick="...">Click to add bananas</div>
</div>
*/

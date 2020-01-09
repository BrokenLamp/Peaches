extern crate peaches;
#[macro_use]
extern crate peaches-macros;

use formulate_trait::Formulate;

#[derive(Formulate)]
struct Pancakes;

#[test]
fn it_works() {
    assert_eq!(Pancakes::formulate(), "Hello, Macro! My name is Pancakes");
}

struct ComponentName {
    state: (String, f32),
    element: Element,
    p_el_0: Option<Box<SomeOtherComponent>>,
}

impl ComponentName {
    pub fn new(state: Option<ComponentState>, some_prop: String, some_optional_prop: Option<String>) -> CompnonentName {

    }
}

impl Component for ComponentName {
    fn render(ctx: &mut GraphicsContext) {

    }
}

// And with Peaches macros

component!(ComponentName(some_prop: String, some_optional_prop: Option<String>) => {
    let (name, set_name) = use_state!(String, "some name");
    let (age, set_age) = use_state!(f32, 28.);
    let styles = use_styles!("./some_css_file.css");
    div!(class=styles.ComponentName {
        div!(class=styles.Name { name });
        div!(class={styles.Age} { age });
        com!(SomeOtherComponent some_prop=6);
        if condition {
            div!({ "Result A" });
        } else {
            div!({ "Result B" });
        }
    });
});

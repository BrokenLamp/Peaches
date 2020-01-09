# Peaches

A React-like GUI engine for making performant desktop applications.

WIP. This does not build.

## Hello World Comonent

```rust
// Our stateless component
component!(App() => {
    let styles = use_styles!(App ["#37474F", "#009688"]);

    <View class={styles.root}>
        <Text value={"Hello World!"} class={styles.hello_world} />
    </View>
});

// Fancy CSS
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
```

## Component Breakdown

```rust
component!(ComponentName(
    prop1: Type,
    prop2: Type,
) => {
    let styles = use_styles!(ComponentName [arg1, arg2]);

    let state1 = use_state!(default_value1);
    let state2 = use_state!(default_value2);

    use_effect!(arg1, arg2 {
        an_async_task(|new_value| set!(state1 => new_value));
    });

    <View class={styles.root}> //  │ we must define one root component to return
        <SomeOtherComponent     // └┐
            prop1={props.prop1?} // └┐ if `props.prop1` is `None`, the entire component will fail and return `None`
            prop2?={props.prop2} //  │ if `props.prop2` is `None`, we'll just pass None to the other component
            prop3={state1}       //  │ set `prop2` to `Some(state1)`
        >                        //  ┤
            <AnotherComponent /> //  │ we can pass children to other components
            {state2}             //  │ some components can be stored in variables
                                 // ┌┘ ── note: this must be an `Option<Box<dyn Component>>`
        </SomeOtherComponent>   //  │ ::<>
        <Button
            on_click={|| set!(state2 => some_other_value)} // we can change state on a component event
        >
            <Text value={"Some Button"} />
        </Button>
    </View>   // This "RSX" will evaluate to an `Option<Box<dyn Component>>`
});
```

#[macro_export]
macro_rules! init_math {
    ($lua:expr, $canvas:expr) => {
        $lua.set(
            "abs",
            hlua::function1(|num: f32| {
                unimplemented!();
            }),
        );

        $lua.set(
            "sin",
            hlua::function1(|angle: f32| {
                unimplemented!();
            }),
        );

        $lua.set(
            "cos",
            hlua::function1(|angle: f32| {
                unimplemented!();
            }),
        );

        $lua.set(
            "flr",
            hlua::function1(|num: f32| {
                unimplemented!();
            }),
        );

        $lua.set(
            "max",
            hlua::function1(|first: f32, second: f32| {
                unimplemented!();
            }),
        );

        $lua.set(
            "min",
            hlua::function1(|first: f32, second: f32| {
                unimplemented!();
            }),
        );

        $lua.set(
            "rnd",
            hlua::function1(|max: f32| {
                unimplemented!();
            }),
        );
    };
}

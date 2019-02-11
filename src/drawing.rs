#[macro_export]
macro_rules! init_drawing {
    ($lua:expr, $canvas:expr) => {
        $lua.set(
            "cls",
            hlua::function0(|| {
                $canvas.borrow_mut().clear();
            }),
        );

        $lua.set(
            "spr",
            hlua::function7(
                |n: u32, x: u32, y: u32, w: u32, h: u32, flip_x: bool, flip_y: bool| {
                    unimplemented!();
                },
            ),
        );

        $lua.set(
            "pset",
            hlua::function3(|x: u32, y: u32, c: u32| {
                unimplemented!();
            }),
        );
    };
}

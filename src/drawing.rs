#[macro_export]
macro_rules! init_drawing {
    ($lua:expr, $canvas:expr) => {
        $lua.set(
            "cls",
            hlua::function0(|| {
                $canvas.borrow_mut().clear();
            }),
        );
    };
}

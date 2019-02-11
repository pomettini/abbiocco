#[macro_export]
macro_rules! init_primitives {
    ($lua:expr, $canvas:expr) => {
        $lua.set(
            "rect",
            hlua::function5(|x0: i32, y0: i32, x1: u32, y1: u32, c: u32| {
                $canvas
                    .borrow_mut()
                    .set_draw_color(colors::COLOR[c as usize]);
                $canvas
                    .borrow_mut()
                    .draw_rect(Rect::new(x0, y0, x1 - x0 as u32, y1 - y0 as u32))
                    .unwrap();
            }),
        );

        $lua.set(
            "rectfill",
            hlua::function5(|x0: i32, y0: i32, x1: u32, y1: u32, c: u32| {
                $canvas
                    .borrow_mut()
                    .set_draw_color(colors::COLOR[c as usize]);
                $canvas
                    .borrow_mut()
                    .fill_rect(Rect::new(x0, y0, x1 - x0 as u32, y1 - y0 as u32))
                    .unwrap();
            }),
        );

        $lua.set(
            "circ",
            hlua::function4(|x: i32, y: i32, r: u32, c: u32| {
                unimplemented!();
            }),
        );

        $lua.set(
            "circfill",
            hlua::function4(|x: i32, y: i32, r: u32, c: u32| {
                unimplemented!();
            }),
        );

        $lua.set(
            "circ",
            hlua::function4(|x: i32, y: i32, r: u32, c: u32| {
                unimplemented!();
            }),
        );

        $lua.set(
            "line",
            hlua::function5(|x0: u32, y0: u32, x1: u32, y1: u32, c: u32| {
                unimplemented!();
            }),
        );
    };
}

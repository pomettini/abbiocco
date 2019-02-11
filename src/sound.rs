#[macro_export]
macro_rules! init_sound {
    ($lua:expr, $canvas:expr) => {
        $lua.set(
            "sfx",
            hlua::function4(|n: u32, channel: u32, offset: u32, length: u32| {
                unimplemented!();
            }),
        );

        $lua.set(
            "music",
            hlua::function3(|n: u32, fadems: u32, channelmask: u32| {
                unimplemented!();
            }),
        );
    };
}

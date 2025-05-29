use crate::engine::prelude::*;

pub fn main() {
    // Example embedded Lua script
    let script = Script::new(include_str!("assets/scripts/hello.lua"), true);

    loop {
        script.update();
    }
}
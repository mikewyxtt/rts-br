use mlua::{Lua, Function, Table};

pub struct Script {
    lua: Lua,
}

impl Script {
    pub fn new(lua_code: &str, autostart: bool) -> Self {
        let lua = Lua::new();
        {
            // Load and execute the script
            lua.load(lua_code).exec().unwrap();

            if autostart {
                if let Ok(start_fn) = lua.globals().get::<_, Function>("start") {
                    let _ = start_fn.call::<(), ()>(());
                }
            }
        }
        Script { lua }
    }

    pub fn start(&self) {
        if let Ok(start_fn) = self.lua.globals().get::<_, Function>("start") {
                let _ = start_fn.call::<(), ()>(());
        }
    }

    pub fn update(&self) {
        if let Ok(update_fn) = self.lua.globals().get::<_, Function>("update") {
            let _ = update_fn.call::<(), ()>(());
        }
    }

    // TODO:
    // pub fn destroy(&self) {
    //     if let Ok(destroy_fn) = self.lua.globals().get::<_, Function>("onDestroy") {
    //         let _ = destroy_fn.call::<(), ()>(());
    //     }
    // }
}

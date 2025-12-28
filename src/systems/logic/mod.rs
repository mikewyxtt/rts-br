use bevy::prelude::*;
use mlua::Lua;

pub struct LuaVM {
    lua: Lua,
}

impl LuaVM {
    pub fn new() -> Self {
        Self { lua: Lua::new() }
    }
}

#[derive(Component)]
pub struct Script {
    code: String,
    started: bool,
}

impl Script {
    pub fn new(path: &str) -> Self {
        Self {
            code: std::fs::read_to_string(path).expect("Failed to load Lua script"),
            started: false,
        }
    }

    /// Initializes the script by calling its Start function.
    pub fn start(&mut self, lua: &Lua) {
        lua.load(&self.code).exec().unwrap();
        if let Ok(start) = lua.globals().get::<_, mlua::Function>("Start") {
            start.call::<_, ()>(()).ok();
        }
        self.started = true;
    }

    /// Calls the Update function of the script. Injects the given delta time.
    pub fn update(&self, lua: &Lua, delta_time: f32) {
        lua.load(&self.code).exec().unwrap();
        if let Ok(update) = lua.globals().get::<_, mlua::Function>("Update") {
            update.call::<_, ()>(delta_time).ok();
        }
    }

    /// Calls the FixedUpdate function of the script.
    pub fn fixed_update(&self, lua: &Lua) {
        lua.load(&self.code).exec().unwrap();
        if let Ok(fixed_update) = lua.globals().get::<_, mlua::Function>("FixedUpdate") {
            fixed_update.call::<_, ()>(()).ok();
        }
    }
}

pub fn run_logic(time: Res<Time>, lua_vm: NonSend<LuaVM>, mut query: Query<(Entity, &mut Script)>) {
    let lua = &lua_vm.lua;

    for (_entity, mut script) in &mut query {
        if !script.started {
            script.start(lua);
        }

        script.update(lua, time.delta_secs());
    }
}

pub fn run_fixed_logic(lua_vm: NonSend<LuaVM>, mut query: Query<(Entity, &mut Script)>) {
    let lua = &lua_vm.lua;

    for (_entity, script) in &mut query {
        script.fixed_update(lua);
    }
}

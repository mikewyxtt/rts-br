use bevy::prelude::*;
use mlua::Lua;

pub struct LuaVM {
    lua: Lua,
}

impl LuaVM {
    pub fn new() -> Self {
        Self {
            lua: Lua::new(),
        }
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
}

struct LuaEntityContext {
    entity: Entity,
}

impl mlua::UserData for LuaEntityContext {}

pub fn run_logic(
    time: Res<Time>,
    lua_vm: NonSend<LuaVM>,
    mut query: Query<(Entity, &mut Script)>,
) {
    let lua = &lua_vm.lua;

    for (entity, mut script) in &mut query {

        // Per-entity environment
        let env = lua.globals();

        env.set("self", LuaEntityContext { entity }).unwrap();

        lua.load(&script.code).exec().unwrap();

        if !script.started {
            if let Ok(start) = lua.globals().get::<_, mlua::Function>("Start") {
                start.call::<_, ()>(()).ok();
            }
            script.started = true;
        }

        if let Ok(update) = lua.globals().get::<_, mlua::Function>("Update") {
            update.call::<_, ()>(time.delta_secs()).ok();
        }
    }
}

pub fn run_fixed_logic(
    lua_vm: NonSend<LuaVM>,
    mut query: Query<(Entity, &mut Script)>,
) {
    let lua = &lua_vm.lua;

    for (entity, script) in &mut query {
        if let Ok(fixed_update) = lua.globals().get::<_, mlua::Function>("FixedUpdate") {
            fixed_update.call::<_, ()>(()).ok();
        }
    }
}

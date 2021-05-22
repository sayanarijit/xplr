use crate::config::Config;
use anyhow::bail;
use anyhow::Result;
use mlua::Lua;
use mlua::LuaSerdeExt;

fn resolve_fn_recursive<'lua, 'a>(
    table: &mlua::Table<'lua>,
    mut path: impl Iterator<Item = &'a str>,
) -> Result<mlua::Function<'lua>> {
    if let Some(nxt) = path.next() {
        match table.get(nxt)? {
            mlua::Value::Table(t) => resolve_fn_recursive(&t, path),
            mlua::Value::Function(f) => Ok(f),
            t => bail!("{:?} is not a function", t),
        }
    } else {
        bail!("Invalid path")
    }
}

/// This function resolves paths like `builtin.func_foo`, `custom.func_bar` into lua functions.
pub fn resolve_fn<'lua>(
    globals: &mlua::Table<'lua>,
    path: &str,
) -> Result<mlua::Function<'lua>> {
    let path = format!("xplr.fn.{}", path);
    resolve_fn_recursive(&globals, path.split('.'))
}

// TODO: Remove config input when config.yml is deprecated
/// Used to initialize Lua globals
pub fn init(lua: &Lua, lua_script: &str, config: Config) -> Result<Config> {
    let globals = lua.globals();

    let lua_xplr = lua.create_table()?;
    lua_xplr.set("config", lua.to_value(&config)?)?;

    let lua_xplr_fn = lua.create_table()?;
    let lua_xplr_fn_builtin = lua.create_table()?;
    let lua_xplr_fn_custom = lua.create_table()?;

    lua_xplr_fn.set("builtin", lua_xplr_fn_builtin)?;
    lua_xplr_fn.set("custom", lua_xplr_fn_custom)?;
    lua_xplr.set("fn", lua_xplr_fn)?;
    globals.set("xplr", lua_xplr)?;

    lua.load(&lua_script).set_name("init")?.exec()?;

    let version: String = match globals.get("version").and_then(|v| lua.from_value(v)) {
        Ok(v) => v,
        Err(_) => bail!("'version' must be defined globally in init.lua"),
    };

    let lua_xplr: mlua::Table = globals.get("xplr")?;

    let config: Config = lua.from_value(lua_xplr.get("config")?)?;
    Ok(config.with_version(version))
}

/// Used to extend Lua globals
pub fn extend(lua: &Lua, lua_script: &str) -> Result<Config> {
    lua.load(&lua_script).set_name("init")?.exec()?;

    let globals = lua.globals();
    let version: String = match globals.get("version").and_then(|v| lua.from_value(v)) {
        Ok(v) => v,
        Err(_) => bail!("'version' must be defined globally in init.lua"),
    };

    let lua_xplr: mlua::Table = globals.get("xplr")?;

    let config: Config = lua.from_value(lua_xplr.get("config")?)?;
    Ok(config.with_version(version))
}

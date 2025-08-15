use crate::app::VERSION;
use crate::config::Config;
use crate::config::Hooks;
use anyhow::bail;
use anyhow::Error;
use anyhow::Result;
use mlua::Lua;
use mlua::LuaSerdeExt;
use mlua::SerializeOptions;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;

pub mod util;

const DEFAULT_LUA_SCRIPT: &str = include_str!("../init.lua");
const UPGRADE_GUIDE_LINK: &str = "https://xplr.dev/en/upgrade-guide";

pub fn serialize<T: Serialize + Sized>(
    lua: &mlua::Lua,
    value: &T,
) -> Result<mlua::Value> {
    lua.to_value_with(value, SerializeOptions::new().serialize_none_to_null(false))
        .map_err(Error::from)
}

fn parse_version(version: &str) -> Result<(u16, u16, u16, Option<u16>)> {
    let mut configv = version.split('.');

    let major = configv.next().unwrap_or_default().parse::<u16>()?;
    let minor = configv.next().unwrap_or_default().parse::<u16>()?;
    let patch = configv
        .next()
        .and_then(|s| s.split('-').next())
        .unwrap_or_default()
        .parse::<u16>()?;

    let pre = configv.next().unwrap_or_default().parse::<u16>().ok();

    Ok((major, minor, patch, pre))
}

/// Check the config version and notify users.
pub fn check_version(version: &str, path: &str) -> Result<()> {
    let (rmajor, rminor, rbugfix, rbeta) = parse_version(VERSION)?;
    let (smajor, sminor, sbugfix, sbeta) = parse_version(version)?;

    if rmajor == smajor && rminor >= sminor && rbugfix >= sbugfix && rbeta == sbeta {
        Ok(())
    } else {
        bail!(
            "incompatible script version in: {}. The script version is: {}, the required version is: {}. Visit {}",
            path,
            version,
            VERSION.to_string(),
            UPGRADE_GUIDE_LINK,
        )
    }
}

/// Used to initialize Lua globals
pub fn init(lua: &Lua) -> Result<(Config, Option<Hooks>)> {
    let config = Config::default();
    let globals = lua.globals();

    let util = util::create_table(lua)?;

    let lua_xplr = lua.create_table()?;
    lua_xplr.set("config", serialize(lua, &config)?)?;
    lua_xplr.set("util", util)?;

    let lua_xplr_fn = lua.create_table()?;
    let lua_xplr_fn_builtin = lua.create_table()?;
    let lua_xplr_fn_custom = lua.create_table()?;

    lua_xplr_fn.set("builtin", lua_xplr_fn_builtin)?;
    lua_xplr_fn.set("custom", lua_xplr_fn_custom)?;
    lua_xplr.set("fn", lua_xplr_fn)?;
    globals.set("xplr", lua_xplr)?;

    let hooks: Option<Hooks> = lua
        .load(DEFAULT_LUA_SCRIPT)
        .set_name("xplr init")
        .call(())
        .and_then(|v| lua.from_value(v))?;

    let lua_xplr: mlua::Table = globals.get("xplr")?;
    let config: Config = lua.from_value(lua_xplr.get("config")?)?;
    Ok((config, hooks))
}

/// Used to extend Lua globals
pub fn extend(lua: &Lua, path: &str) -> Result<(Config, Option<Hooks>)> {
    let globals = lua.globals();

    let script = fs::read(path)?;

    let hooks: Option<Hooks> = lua
        .load(&script)
        .set_name(path)
        .call(())
        .and_then(|v| lua.from_value(v))?;

    let version: String = match globals.get("version").and_then(|v| lua.from_value(v)) {
        Ok(v) => v,
        Err(_) => bail!("'version' must be defined globally in {}", path),
    };

    check_version(&version, path)?;

    let lua_xplr: mlua::Table = globals.get("xplr")?;

    let config: Config = lua.from_value(lua_xplr.get("config")?)?;
    Ok((config, hooks))
}

fn resolve_fn_recursive<'a>(
    table: &mlua::Table,
    mut path: impl Iterator<Item = &'a str>,
) -> Result<mlua::Function> {
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
pub fn resolve_fn(globals: &mlua::Table, path: &str) -> Result<mlua::Function> {
    resolve_fn_recursive(globals, path.split('.'))
}

pub fn call<R: DeserializeOwned>(lua: &Lua, func: &str, arg: mlua::Value) -> Result<R> {
    let func = format!("xplr.fn.{func}");
    let func = resolve_fn(&lua.globals(), &func)?;
    let res: mlua::Value = func.call(arg)?;
    let res: R = lua.from_value(res)?;
    Ok(res)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_compatibility() {
        assert!(check_version(VERSION, "foo path").is_ok());

        // Current release if OK
        assert!(check_version("1.0.1", "foo path").is_ok());

        // Prev major release is ERR
        assert!(check_version("0.20.1", "foo path").is_err());

        // Prev minor release is ERR
        assert!(check_version("1.-1.0", "foo path").is_err());

        // Prev bugfix release is OK
        assert!(check_version("1.0.0", "foo path").is_ok());

        // Next major release is ERR
        assert!(check_version("2.0.1", "foo path").is_err());

        // Next minor release is ERR
        assert!(check_version("1.1.1", "foo path").is_err());

        // Next bugfix release is ERR (Change when we get to v1)
        assert!(check_version("1.0.2", "foo path").is_err());
    }
}

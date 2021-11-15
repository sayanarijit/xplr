use crate::app::Node;
use crate::app::VERSION;
use crate::config::Config;
use anyhow::bail;
use anyhow::Result;
use mlua::Lua;
use mlua::LuaSerdeExt;
use serde::Deserialize;
use std::fs;

const DEFAULT_LUA_SCRIPT: &str = include_str!("init.lua");
const INTERNAL_LUA_SCRIPT: &str = include_str!("__cache__.lua");
const UPGRADE_GUIDE_LINK: &str = "https://xplr.dev/en/upgrade-guide.html";

fn parse_version(version: &str) -> Result<(u16, u16, u16, Option<u16>)> {
    let mut configv = version.split('.');

    let major = configv.next().unwrap_or_default().parse::<u16>()?;
    let minor = configv.next().unwrap_or_default().parse::<u16>()?;
    let bugfix = configv
        .next()
        .and_then(|s| s.split('-').next())
        .unwrap_or_default()
        .parse::<u16>()?;

    let beta = configv.next().unwrap_or_default().parse::<u16>().ok();

    Ok((major, minor, bugfix, beta))
}

/// Check the config version and notify users.
pub fn check_version(version: &str, path: &str) -> Result<()> {
    // Until we're v1, let's ignore major versions
    let (rmajor, rminor, rbugfix, rbeta) = parse_version(VERSION)?;
    let (smajor, sminor, sbugfix, sbeta) = parse_version(version)?;

    if rmajor == smajor
        && rminor == sminor
        && rbugfix >= sbugfix
        && rbeta == sbeta
    {
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
pub fn init(lua: &Lua) -> Result<Config> {
    let config = Config::default();
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

    lua.load(DEFAULT_LUA_SCRIPT).set_name("init")?.exec()?;
    lua.load(INTERNAL_LUA_SCRIPT).set_name("internal")?.exec()?;

    let lua_xplr: mlua::Table = globals.get("xplr")?;
    let config: Config = lua.from_value(lua_xplr.get("config")?)?;
    Ok(config)
}

/// Used to extend Lua globals
pub fn extend(lua: &Lua, path: &str) -> Result<Config> {
    let globals = lua.globals();

    let script = fs::read_to_string(path)?;

    lua.load(&script).set_name("init")?.exec()?;

    let version: String =
        match globals.get("version").and_then(|v| lua.from_value(v)) {
            Ok(v) => v,
            Err(_) => bail!("'version' must be defined globally in {}", path),
        };

    check_version(&version, path)?;

    let lua_xplr: mlua::Table = globals.get("xplr")?;

    let config: Config = lua.from_value(lua_xplr.get("config")?)?;
    Ok(config)
}

/// Used to call lua functions.
pub fn call<'lua, R: Deserialize<'lua>>(
    lua: &'lua Lua,
    func: &str,
    arg: mlua::Value<'lua>,
) -> Result<R> {
    let func = format!("xplr.__CACHE__.caller(xplr.fn.{})", func);
    let caller: mlua::Function = lua.load(&func).eval()?;
    let res: mlua::Value = caller.call(arg)?;
    let res: R = lua.from_value(res)?;
    Ok(res)
}

/// Used to cache the directory nodes.
pub fn cache_directory_nodes<'lua>(
    lua: &'lua Lua,
    nodes: &[Node],
) -> Result<()> {
    let func = "xplr.__CACHE__.set_directory_nodes";
    let func: mlua::Function = lua.load(func).eval()?;
    func.call(lua.to_value(nodes)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_compatibility() {
        assert!(check_version(VERSION, "foo path").is_ok());

        // Current release if OK
        assert!(check_version("0.16.3", "foo path").is_ok());

        // Prev major release is ERR
        // - Not yet

        // Prev minor release is ERR (Change when we get to v1)
        assert!(check_version("0.15.3", "foo path").is_err());

        // Prev bugfix release is OK
        assert!(check_version("0.16.2", "foo path").is_ok());

        // Next major release is ERR
        assert!(check_version("1.16.3", "foo path").is_err());

        // Next minor release is ERR
        assert!(check_version("0.17.3", "foo path").is_err());

        // Next bugfix release is ERR (Change when we get to v1)
        assert!(check_version("0.16.4", "foo path").is_err());
    }
}

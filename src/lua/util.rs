use crate::app::VERSION;
use crate::explorer;
use crate::lua;
use crate::msg::in_::external::ExplorerConfig;
use crate::path;
use crate::ui;
use crate::ui::Style;
use anyhow::Result;
use lscolors::LsColors;
use mlua::Error as LuaError;
use mlua::Lua;
use mlua::LuaSerdeExt;
use mlua::Table;
use mlua::Value;
use path_absolutize::*;
use serde::de::Error;
use serde::{Deserialize, Serialize};
use serde_json as json;
use serde_yaml as yaml;
use std::path::PathBuf;
use std::process::Command;

pub(crate) fn create_table(lua: &Lua) -> Result<Table> {
    let mut util = lua.create_table()?;

    util = version(util, lua)?;
    util = dirname(util, lua)?;
    util = basename(util, lua)?;
    util = absolute(util, lua)?;
    util = explore(util, lua)?;
    util = shell_execute(util, lua)?;
    util = shell_quote(util, lua)?;
    util = from_json(util, lua)?;
    util = to_json(util, lua)?;
    util = from_yaml(util, lua)?;
    util = to_yaml(util, lua)?;
    util = lscolor(util, lua)?;
    util = paint(util, lua)?;
    util = relative_to(util, lua)?;
    util = path_shorthand(util, lua)?;

    Ok(util)
}

/// Get the xplr version details.
///
/// Type: function() -> { major: number, minor: number, patch: number }
///
/// Example:
///
/// ```lua
/// xplr.util.version()
/// -- { major = 0, minor = 0, patch = 0 }
/// ```
pub fn version<'a>(util: Table<'a>, lua: &Lua) -> Result<Table<'a>> {
    #[derive(Debug, Default, Serialize, Deserialize)]
    struct Version {
        major: u16,
        minor: u16,
        patch: u16,
    }

    let func = lua.create_function(|lua, ()| {
        let (major, minor, patch, _) =
            lua::parse_version(VERSION).map_err(LuaError::custom)?;

        let version = Version {
            major,
            minor,
            patch,
        };

        let res = lua::serialize(lua, &version).map_err(LuaError::custom)?;
        Ok(res)
    })?;

    util.set("version", func)?;
    Ok(util)
}

/// Get the directory name of a given path.
///
/// Type: function( path:string ) -> path:string|nil
///
/// Example:
///
/// ```lua
/// xplr.util.dirname("/foo/bar")
/// -- "/foo"
/// ```
pub fn dirname<'a>(util: Table<'a>, lua: &Lua) -> Result<Table<'a>> {
    let func = lua.create_function(|_, path: String| {
        let parent = PathBuf::from(path)
            .parent()
            .map(|p| p.to_string_lossy().to_string());
        Ok(parent)
    })?;
    util.set("dirname", func)?;
    Ok(util)
}

/// Get the base name of a given path.
///
/// Type: function( path:string ) -> path:string|nil
///
/// Example:
///
/// ```lua
/// xplr.util.basename("/foo/bar")
/// -- "bar"
/// ```
pub fn basename<'a>(util: Table<'a>, lua: &Lua) -> Result<Table<'a>> {
    let func = lua.create_function(|_, path: String| {
        let parent = PathBuf::from(path)
            .file_name()
            .map(|p| p.to_string_lossy().to_string());
        Ok(parent)
    })?;
    util.set("basename", func)?;
    Ok(util)
}

/// Get the absolute path of the given path by prepending $PWD.
/// It doesn't check if the path exists.
///
/// Type: function( path:string ) -> path:string
///
/// Example:
///
/// ```lua
/// xplr.util.absolute("foo/bar")
/// -- "/tmp/foo/bar"
/// ```
pub fn absolute<'a>(util: Table<'a>, lua: &Lua) -> Result<Table<'a>> {
    let func = lua.create_function(|_, path: String| {
        let parent = PathBuf::from(path)
            .absolutize()?
            .to_string_lossy()
            .to_string();
        Ok(parent)
    })?;
    util.set("absolute", func)?;
    Ok(util)
}

/// Explore directories with the given explorer config.
///
/// Type: function( path:string, [ExplorerConfig][1]|nil ) -> { [Node][2], ... }
///
/// Example:
///
/// ```lua
///
/// xplr.util.explore("/tmp")
/// -- { { absolute_path = "/tmp/a", ... }, ... }
///
/// xplr.util.explore("/tmp", app.explorer_config)
/// -- { { absolute_path = "/tmp/a", ... }, ... }
/// ```
///
/// [1]: https://xplr.dev/en/lua-function-calls#explorer-config
/// [2]: https://xplr.dev/en/lua-function-calls#node
pub fn explore<'a>(util: Table<'a>, lua: &Lua) -> Result<Table<'a>> {
    let func = lua.create_function(|lua, (path, config): (String, Option<Table>)| {
        let config: ExplorerConfig = if let Some(cfg) = config {
            lua.from_value(Value::Table(cfg))?
        } else {
            ExplorerConfig::default()
        };

        let nodes = explorer::explore(&PathBuf::from(path), &config)
            .map_err(LuaError::custom)?;
        let res = lua::serialize(lua, &nodes).map_err(LuaError::custom)?;
        Ok(res)
    })?;
    util.set("explore", func)?;
    Ok(util)
}

/// Execute shell commands safely.
///
/// Type: function( program:string, args:{ string, ... }|nil ) -> { stdout = string, stderr = string, returncode = number|nil }
///
/// Example:
///
/// ```lua
/// xplr.util.shell_execute("pwd")
/// -- "/present/working/directory"
///
/// xplr.util.shell_execute("bash", {"-c", "xplr --help"})
/// -- { stdout = "xplr...", stderr = "", returncode = 0 }
/// ```
pub fn shell_execute<'a>(util: Table<'a>, lua: &Lua) -> Result<Table<'a>> {
    let func =
        lua.create_function(|lua, (program, args): (String, Option<Vec<String>>)| {
            let mut cmd = Command::new(program);
            let mut cmd_ref = &mut cmd;
            if let Some(args) = args {
                cmd_ref = cmd_ref.args(args)
            };
            let output = cmd_ref.output()?;

            let res = lua.create_table()?;
            res.set("stdout", String::from_utf8_lossy(&output.stdout))?;
            res.set("stderr", String::from_utf8_lossy(&output.stderr))?;
            res.set("returncode", output.status.code())?;
            Ok(res)
        })?;
    util.set("shell_execute", func)?;
    Ok(util)
}

/// Quote commands and paths safely.
///
/// Type: function( string ) -> string
///
/// Example:
///
/// ```lua
/// xplr.util.shell_quote("a'b\"c")
/// -- 'a'"'"'b"c'
/// ```
pub fn shell_quote<'a>(util: Table<'a>, lua: &Lua) -> Result<Table<'a>> {
    let func = lua.create_function(|_, string: String| {
        Ok(format!("'{}'", string.replace('\'', r#"'"'"'"#)))
    })?;
    util.set("shell_quote", func)?;
    Ok(util)
}

/// Load JSON string into Lua value.
///
/// Type: function( string ) -> any
///
/// Example:
///
/// ```lua
/// xplr.util.from_json([[{"foo": "bar"}]])
/// -- { foo = "bar" }
/// ```
pub fn from_json<'a>(util: Table<'a>, lua: &Lua) -> Result<Table<'a>> {
    let func = lua.create_function(|lua, string: String| {
        let val = json::from_str::<yaml::Value>(&string).map_err(LuaError::custom)?;
        lua::serialize(lua, &val).map_err(Error::custom)
    })?;
    util.set("from_json", func)?;
    Ok(util)
}

/// Dump Lua value into JSON (i.e. also YAML) string.
///
/// Type: function( value ) -> string
///
/// Example:
///
/// ```lua
/// xplr.util.to_json({ foo = "bar" })
/// -- [[{ "foo": "bar" }]]
///
/// xplr.util.to_json({ foo = "bar" }, { pretty = true })
/// -- [[{
/// --   "foo": "bar"
/// -- }]]
/// ```
pub fn to_json<'a>(util: Table<'a>, lua: &Lua) -> Result<Table<'a>> {
    #[derive(Debug, Default, Serialize, Deserialize)]
    struct Options {
        pretty: bool,
    }

    let func =
        lua.create_function(|lua, (value, options): (Value, Option<Table>)| {
            let options: Options = if let Some(o) = options {
                lua.from_value(Value::Table(o))?
            } else {
                Default::default()
            };

            if options.pretty {
                json::to_string_pretty(&value).map_err(Error::custom)
            } else {
                json::to_string(&value).map_err(Error::custom)
            }
        })?;
    util.set("to_json", func)?;
    Ok(util)
}

/// Load YAML (i.e. also JSON) string into Lua value.
///
/// Type: function( string ) -> value
///
/// Example:
///
/// ```lua
/// xplr.util.from_yaml([[{foo: bar}]])
/// -- { foo = "bar" }
/// ```
pub fn from_yaml<'a>(util: Table<'a>, lua: &Lua) -> Result<Table<'a>> {
    let func = lua.create_function(|lua, string: String| {
        let val = yaml::from_str::<yaml::Value>(&string).map_err(LuaError::custom)?;
        lua::serialize(lua, &val).map_err(Error::custom)
    })?;
    util.set("from_yaml", func)?;
    Ok(util)
}

/// Dump Lua value into YAML string.
///
/// Type: function( value ) -> string
///
/// Example:
///
/// ```lua
/// xplr.util.to_yaml({ foo = "bar" })
/// -- "foo: bar"
/// ```
pub fn to_yaml<'a>(util: Table<'a>, lua: &Lua) -> Result<Table<'a>> {
    #[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
    pub struct Options {
        pretty: bool,
    }

    let func = lua.create_function(|_, value: Value| {
        yaml::to_string(&value).map_err(Error::custom)
    })?;
    util.set("to_yaml", func)?;
    Ok(util)
}

/// Get a [Style][3] object for the given path based on the LS_COLORS
/// environment variable.
///
/// Type: function( path:string ) -> Style[3]|nil
///
/// Example:
///
/// ```lua
/// xplr.util.lscolor("Desktop")
/// -- { fg = "Red", bg = nil, add_modifiers = {}, sub_modifiers = {} }
/// ```
///
/// [3]: https://xplr.dev/en/style
pub fn lscolor<'a>(util: Table<'a>, lua: &Lua) -> Result<Table<'a>> {
    let lscolors = LsColors::from_env().unwrap_or_default();
    let func = lua.create_function(move |lua, path: String| {
        if *ui::NO_COLOR {
            return Ok(mlua::Nil);
        }

        let style = lscolors.style_for_path(path).map(Style::from);
        lua::serialize(lua, &style).map_err(LuaError::custom)
    })?;
    util.set("lscolor", func)?;
    Ok(util)
}

/// Apply style (escape sequence) to string using a given [Style][3] object.
///
/// Type: function( string, [Style][3]|nil ) -> string
///
/// Example:
///
/// ```lua
/// xplr.util.paint("Desktop", { fg = "Red", bg = nil, add_modifiers = {}, sub_modifiers = {} })
/// -- "\u001b[31mDesktop\u001b[0m"
/// ```
pub fn paint<'a>(util: Table<'a>, lua: &Lua) -> Result<Table<'a>> {
    let func =
        lua.create_function(|lua, (string, style): (String, Option<Table>)| {
            if *ui::NO_COLOR {
                return Ok(string);
            }

            let Some(style) = style else {
                return Ok(string);
            };

            let style: Style = lua.from_value(Value::Table(style))?;
            let ansi_style: nu_ansi_term::Style = style.into();
            Ok::<String, LuaError>(ansi_style.paint(string).to_string())
        })?;
    util.set("paint", func)?;
    Ok(util)
}

/// Get the relative path based on the given base path or current working dir.
/// Will error if it fails to determine a relative path.
///
/// Type: function( path:string, base:string ) -> path:string
///
/// Example:
///
/// ```lua
/// xplr.util.relative_to("/present/working/directory")
/// -- "."
///
/// xplr.util.relative_to("/present/working")
/// -- ".."
///
/// xplr.util.relative_to("/present/working/directory", "/present/foo/bar")
/// -- "../../working/directory"
/// ```
pub fn relative_to<'a>(util: Table<'a>, lua: &Lua) -> Result<Table<'a>> {
    let func = lua.create_function(|_, (path, base): (String, Option<String>)| {
        path::relative_to(path, base)
            .map(|p| p.to_string_lossy().to_string())
            .map_err(LuaError::custom)
    })?;
    util.set("relative_to", func)?;
    Ok(util)
}

/// Display the given path in shorthand form using the following rules:
/// - either relative to your home dir if it makes sense
/// - or relative to the optional base path / current working directory
/// - or absolute path if it makes the most sense
///
/// Type: function( path:string, base:string|nil ) -> path:string|nil
///
/// Example:
///
/// ```lua
/// xplr.util.path_shorthand("/home/username/.config")
/// -- "~/.config"
///
/// xplr.util.path_shorthand("/present/working/directory")
/// -- "../directory"
///
/// xplr.util.path_shorthand("/present/working/directory", "/present/foo/bar")
/// -- "../../working/directory"
/// ```
pub fn path_shorthand<'a>(util: Table<'a>, lua: &Lua) -> Result<Table<'a>> {
    let func =
        lua.create_function(move |_, (path, base): (String, Option<String>)| {
            path::shorthand(path, base).map_err(LuaError::custom)
        })?;
    util.set("path_shorthand", func)?;
    Ok(util)
}

use crate::app::VERSION;
use crate::config::NodeTypesConfig;
use crate::explorer;
use crate::lua;
use crate::msg::in_::external::ExplorerConfig;
use crate::node::Node;
use crate::path;
use crate::path::RelativityConfig;
use crate::permissions::Octal;
use crate::permissions::Permissions;
use crate::ui;
use crate::ui::Layout;
use crate::ui::Style;
use crate::ui::WrapOptions;
use anyhow::Result;
use lazy_static::lazy_static;
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
use std::borrow::Cow;
use std::path::PathBuf;
use std::process::Command;

lazy_static! {
    static ref LS_COLORS: LsColors = LsColors::from_env().unwrap_or_default();
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
pub fn version(util: Table, lua: &Lua) -> Result<Table> {
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

/// Print the given value to the console, and return it as a string.
/// Useful for debugging.
///
/// Type: function( value ) -> string
///
/// Example:
///
/// ```lua
/// xplr.util.debug({ foo = "bar", bar = function() end })
/// -- {
/// --   ["bar"] = function: 0x55e5cebdeae0,
/// --   ["foo"] = "bar",
/// -- }
/// ```
pub fn debug(util: Table, lua: &Lua) -> Result<Table> {
    let func = lua.create_function(|_, value: Value| {
        let log = format!("{value:#?}");
        println!("{log}");
        Ok(log)
    })?;
    util.set("debug", func)?;
    Ok(util)
}

/// Clone/deepcopy a Lua value. Doesn't work with functions.
///
/// Type: function( value ) -> value
///
/// Example:
///
/// ```lua
/// local val = { foo = "bar" }
/// local val_clone = xplr.util.clone(val)
/// val.foo = "baz"
/// print(val_clone.foo)
/// -- "bar"
/// ```
pub fn clone(util: Table, lua: &Lua) -> Result<Table> {
    let func = lua.create_function(move |lua, value: Value| {
        lua::serialize(lua, &value).map_err(LuaError::custom)
    })?;
    util.set("clone", func)?;
    Ok(util)
}

/// Check if the given path exists.
///
/// Type: function( path:string ) -> boolean
///
/// Example:
///
/// ```lua
/// xplr.util.exists("/foo/bar")
/// -- true
/// ```
pub fn exists(util: Table, lua: &Lua) -> Result<Table> {
    let func =
        lua.create_function(move |_, path: String| Ok(PathBuf::from(path).exists()))?;
    util.set("exists", func)?;
    Ok(util)
}

/// Check if the given path is a directory.
///
/// Type: function( path:string ) -> boolean
///
/// Example:
///
/// ```lua
/// xplr.util.is_dir("/foo/bar")
/// -- true
/// ```
pub fn is_dir(util: Table, lua: &Lua) -> Result<Table> {
    let func =
        lua.create_function(move |_, path: String| Ok(PathBuf::from(path).is_dir()))?;
    util.set("is_dir", func)?;
    Ok(util)
}

/// Check if the given path is a file.
///
/// Type: function( path:string ) -> boolean
///
/// Example:
///
/// ```lua
/// xplr.util.is_file("/foo/bar")
/// -- true
/// ```
pub fn is_file(util: Table, lua: &Lua) -> Result<Table> {
    let func =
        lua.create_function(move |_, path: String| Ok(PathBuf::from(path).is_file()))?;
    util.set("is_file", func)?;
    Ok(util)
}

/// Check if the given path is a symlink.
///
/// Type: function( path:string ) -> boolean
///
/// Example:
///
/// ```lua
/// xplr.util.is_file("/foo/bar")
/// -- true
/// ```
pub fn is_symlink(util: Table, lua: &Lua) -> Result<Table> {
    let func = lua
        .create_function(move |_, path: String| Ok(PathBuf::from(path).is_symlink()))?;
    util.set("is_symlink", func)?;
    Ok(util)
}

/// Check if the given path is an absolute path.
///
/// Type: function( path:string ) -> boolean
///
/// Example:
///
/// ```lua
/// xplr.util.is_absolute("/foo/bar")
/// -- true
/// ```
pub fn is_absolute(util: Table, lua: &Lua) -> Result<Table> {
    let func = lua
        .create_function(move |_, path: String| Ok(PathBuf::from(path).is_absolute()))?;
    util.set("is_absolute", func)?;
    Ok(util)
}

/// Split a path into its components.
///
/// Type: function( path:string ) -> boolean
///
/// Example:
///
/// ```lua
/// xplr.util.path_split("/foo/bar")
/// -- { "/", "foo", "bar" }
///
/// xplr.util.path_split(".././foo")
/// -- { "..", "foo" }
/// ```
pub fn path_split(util: Table, lua: &Lua) -> Result<Table> {
    let func = lua.create_function(move |_, path: String| {
        let components: Vec<String> = PathBuf::from(path)
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect();
        Ok(components)
    })?;
    util.set("path_split", func)?;
    Ok(util)
}

/// Get [Node][5] information of a given path.
/// Doesn't check if the path exists.
/// Returns nil if the path is "/".
/// Errors out if absolute path can't be obtained.
///
/// Type: function( path:string ) -> [Node][5]|nil
///
/// Example:
///
/// ```lua
/// xplr.util.node("./bar")
/// -- { parent = "/pwd", relative_path = "bar", absolute_path = "/pwd/bar", ... }
///
/// xplr.util.node("/")
/// -- nil
/// ```
pub fn node(util: Table, lua: &Lua) -> Result<Table> {
    let func = lua.create_function(move |lua, path: String| {
        let path = PathBuf::from(path);
        let abs = path.absolutize()?;
        match (abs.parent(), abs.file_name()) {
            (Some(parent), Some(name)) => {
                let node = Node::new(
                    parent.to_string_lossy().to_string(),
                    name.to_string_lossy().to_string(),
                );
                Ok(lua::serialize(lua, &node).map_err(LuaError::custom)?)
            }
            (_, _) => Ok(Value::Nil),
        }
    })?;
    util.set("node", func)?;
    Ok(util)
}

/// Get the configured [Node Type][6] of a given [Node][5].
///
/// Type: function( [Node][5], [xplr.config.node_types][7]|nil ) -> [Node Type][6]
///
/// If the second argument is missing, global config `xplr.config.node_types`
/// will be used.
///
/// Example:
///
/// ```lua
/// xplr.util.node_type(app.focused_node)
/// -- { style = { fg = "Red", ... }, meta = { icon = "", ... } ... }
///
/// xplr.util.node_type(xplr.util.node("/foo/bar"), xplr.config.node_types)
/// -- { style = { fg = "Red", ... }, meta = { icon = "", ... } ... }
/// ```
pub fn node_type(util: Table, lua: &Lua) -> Result<Table> {
    let func =
        lua.create_function(move |lua, (node, config): (Table, Option<Table>)| {
            let node: Node = lua.from_value(Value::Table(node))?;
            let config: Table = if let Some(config) = config {
                config
            } else {
                lua.globals()
                    .get::<Table>("xplr")?
                    .get::<Table>("config")?
                    .get::<Table>("node_types")?
            };
            let config: NodeTypesConfig = lua.from_value(Value::Table(config))?;
            let node_type = config.get(&node);
            let node_type = lua::serialize(lua, &node_type).map_err(LuaError::custom)?;
            Ok(node_type)
        })?;
    util.set("node_type", func)?;
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
pub fn dirname(util: Table, lua: &Lua) -> Result<Table> {
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
pub fn basename(util: Table, lua: &Lua) -> Result<Table> {
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
pub fn absolute(util: Table, lua: &Lua) -> Result<Table> {
    let func = lua.create_function(|_, path: String| {
        let abs = PathBuf::from(path)
            .absolutize()?
            .to_string_lossy()
            .to_string();
        Ok(abs)
    })?;
    util.set("absolute", func)?;
    Ok(util)
}

/// Get the relative path based on the given base path or current working dir.
/// Will error if it fails to determine a relative path.
///
/// Type: function( path:string, options:table|nil ) -> path:string
///
/// Options type: { base:string|nil, with_prefix_dots:bookean|nil, without_suffix_dots:boolean|nil }
///
/// - If `base` path is given, the path will be relative to it.
/// - If `with_prefix_dots` is true, the path will always start with dots `..` / `.`
/// - If `without_suffix_dots` is true, the name will be visible instead of dots `..` / `.`
///
/// Example:
///
/// ```lua
/// xplr.util.relative_to("/present/working/directory")
/// -- "."
///
/// xplr.util.relative_to("/present/working/directory/foo")
/// -- "foo"
///
/// xplr.util.relative_to("/present/working/directory/foo", { with_prefix_dots = true })
/// -- "./foo"
///
/// xplr.util.relative_to("/present/working/directory", { without_suffix_dots = true })
/// -- "../directory"
///
/// xplr.util.relative_to("/present/working")
/// -- ".."
///
/// xplr.util.relative_to("/present/working", { without_suffix_dots = true })
/// -- "../../working"
///
/// xplr.util.relative_to("/present/working/directory", { base = "/present/foo/bar" })
/// -- "../../working/directory"
/// ```
pub fn relative_to(util: Table, lua: &Lua) -> Result<Table> {
    let func = lua.create_function(|lua, (path, config): (String, Option<Table>)| {
        let config: Option<RelativityConfig<String>> =
            lua.from_value(config.map(Value::Table).unwrap_or(Value::Nil))?;
        path::relative_to(path, config.as_ref())
            .map(|p| p.to_string_lossy().to_string())
            .map_err(LuaError::custom)
    })?;
    util.set("relative_to", func)?;
    Ok(util)
}

/// Shorten the given absolute path using the following rules:
/// - either relative to your home dir if it makes sense
/// - or relative to the current working directory
/// - or absolute path if it makes the most sense
///
/// Type: Similar to `xplr.util.relative_to`
///
/// Example:
///
/// ```lua
/// xplr.util.shorten("/home/username/.config")
/// -- "~/.config"
///
/// xplr.util.shorten("/present/working/directory")
/// -- "."
///
/// xplr.util.shorten("/present/working/directory/foo")
/// -- "foo"
///
/// xplr.util.shorten("/present/working/directory/foo", { with_prefix_dots = true })
/// -- "./foo"
///
/// xplr.util.shorten("/present/working/directory", { without_suffix_dots = true })
/// -- "../directory"
///
/// xplr.util.shorten("/present/working/directory", { base = "/present/foo/bar" })
/// -- "../../working/directory"
///
/// xplr.util.shorten("/tmp")
/// -- "/tmp"
/// ```
pub fn shorten(util: Table, lua: &Lua) -> Result<Table> {
    let func =
        lua.create_function(move |lua, (path, config): (String, Option<Table>)| {
            let config: Option<RelativityConfig<String>> =
                lua.from_value(config.map(Value::Table).unwrap_or(Value::Nil))?;
            path::shorten(path, config.as_ref()).map_err(LuaError::custom)
        })?;
    util.set("shorten", func)?;
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
pub fn explore(util: Table, lua: &Lua) -> Result<Table> {
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
/// -- { stdout = "/present/working/directory", stderr = "", returncode = 0 }
///
/// xplr.util.shell_execute("bash", {"-c", "xplr --help"})
/// -- { stdout = "xplr...", stderr = "", returncode = 0 }
/// ```
pub fn shell_execute(util: Table, lua: &Lua) -> Result<Table> {
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
pub fn shell_quote(util: Table, lua: &Lua) -> Result<Table> {
    let func = lua.create_function(|_, string: String| {
        Ok(format!("'{}'", string.replace('\'', r#"'"'"'"#)))
    })?;
    util.set("shell_quote", func)?;
    Ok(util)
}

/// Escape commands and paths safely.
///
/// Type: function( string ) -> string
///
/// Example:
///
/// ```lua
/// xplr.util.shell_escape("a'b\"c")
/// -- "\"a'b\\\"c\""
/// ```
pub fn shell_escape(util: Table, lua: &Lua) -> Result<Table> {
    let func = lua.create_function(move |_, string: String| {
        let val = path::escape(&string).to_string();
        Ok(val)
    })?;
    util.set("shell_escape", func)?;
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
pub fn from_json(util: Table, lua: &Lua) -> Result<Table> {
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
pub fn to_json(util: Table, lua: &Lua) -> Result<Table> {
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
pub fn from_yaml(util: Table, lua: &Lua) -> Result<Table> {
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
pub fn to_yaml(util: Table, lua: &Lua) -> Result<Table> {
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
/// Type: function( path:string ) -> [Style][3]
///
/// Example:
///
/// ```lua
/// xplr.util.lscolor("Desktop")
/// -- { fg = "Red", bg = nil, add_modifiers = {}, sub_modifiers = {} }
/// ```
pub fn lscolor(util: Table, lua: &Lua) -> Result<Table> {
    let func = lua.create_function(move |lua, path: String| {
        let style = LS_COLORS
            .style_for_path(path)
            .map(Style::from)
            .unwrap_or_default();
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
pub fn paint(util: Table, lua: &Lua) -> Result<Table> {
    let func =
        lua.create_function(|lua, (string, style): (String, Option<Table>)| {
            if *ui::NO_COLOR {
                return Ok(string);
            }

            if let Some(style) = style {
                let style: Style = lua.from_value(Value::Table(style))?;
                let ansi_style: nu_ansi_term::Style = style.into();
                Ok::<String, LuaError>(ansi_style.paint(string).to_string())
            } else {
                Ok(string)
            }
        })?;
    util.set("paint", func)?;
    Ok(util)
}

/// Mix multiple [Style][3] objects into one.
///
/// Type: function( { [Style][3], [Style][3], ... } ) -> [Style][3]
///
/// Example:
///
/// ```lua
/// xplr.util.style_mix({{ fg = "Red" }, { bg = "Blue" }, { add_modifiers = {"Bold"} }})
/// -- { fg = "Red", bg = "Blue", add_modifiers = { "Bold" }, sub_modifiers = {} }
/// ```
pub fn style_mix(util: Table, lua: &Lua) -> Result<Table> {
    let func = lua.create_function(|lua, styles: Vec<Table>| {
        let mut style = Style::default();
        for other in styles {
            let other: Style = lua.from_value(Value::Table(other))?;
            style = style.extend(&other);
        }

        lua::serialize(lua, &style).map_err(LuaError::custom)
    })?;
    util.set("style_mix", func)?;
    Ok(util)
}

/// Wrap the given text to fit the specified width.
/// It will try to not split words when possible.
///
/// Type: function( string, options:number|table ) -> { string, ...}
///
/// Options type: { width = number, initial_indent = string|nil, subsequent_indent = string|nil, break_words = boolean|nil }
///
/// Example:
///
/// ```lua
/// xplr.util.textwrap("this will be cut off", 11)
/// -- { "this will', 'be cut off" }
///
/// xplr.util.textwrap(
///   "this will be cut off",
///   { width = 12, initial_indent = "", subsequent_indent = "    ", break_words = false }
/// )
/// -- { "this will be", "    cut off" }
/// ```
pub fn textwrap(util: Table, lua: &Lua) -> Result<Table> {
    let func = lua.create_function(|lua, (text, options): (String, Value)| {
        let lines = match lua.from_value::<usize>(options.clone()) {
            Ok(width) => textwrap::wrap(&text, width),
            Err(_) => {
                let options = lua.from_value::<WrapOptions>(options)?;
                textwrap::wrap(&text, options.get_options())
            }
        };

        Ok(lines.iter().map(Cow::to_string).collect::<Vec<String>>())
    })?;
    util.set("textwrap", func)?;
    Ok(util)
}

/// Find the target layout in the given layout and replace it with the replacement layout,
/// returning a new layout.
///
/// Type: function( layout:[Layout][4], target:[Layout][4], replacement:[Layout][4] ) -> layout:[Layout][4]
///
/// Example:
///
/// ```lua
/// local layout = {
///   Horizontal = {
///     splits = {
///       "Table",  -- Target
///       "HelpMenu",
///     },
///     config = ...,
///   }
/// }
///
/// xplr.util.layout_replace(layout, "Table", "Selection")
/// -- {
/// --   Horizontal = {
/// --     splits = {
/// --       "Selection",  -- Replacement
/// --       "HelpMenu",
/// --     },
/// --     config = ...
/// --   }
/// -- }
/// ```
pub fn layout_replace(util: Table, lua: &Lua) -> Result<Table> {
    let func = lua.create_function(
        move |lua, (layout, target, replacement): (Value, Value, Value)| {
            let layout: Layout = lua.from_value(layout)?;
            let target: Layout = lua.from_value(target)?;
            let replacement: Layout = lua.from_value(replacement)?;

            let res = layout.replace(&target, &replacement);
            let res = lua::serialize(lua, &res).map_err(LuaError::custom)?;

            Ok(res)
        },
    )?;
    util.set("layout_replace", func)?;
    Ok(util)
}

/// Convert [Permission][8] to rwxrwxrwx representation with special bits.
///
/// Type: function( [Permission][8] ) -> string
///
/// Example:
///
/// ```lua
/// xplr.util.permissions_rwx({ user_read = true })
/// -- "r--------"
///
/// xplr.util.permissions_rwx(app.focused_node.permission)
/// -- "rwxrwsrwT"
/// ```
pub fn permissions_rwx(util: Table, lua: &Lua) -> Result<Table> {
    let func = lua.create_function(|lua, permission: Table| {
        let permissions: Permissions = lua.from_value(Value::Table(permission))?;
        let permissions = permissions.to_string();
        Ok(permissions)
    })?;
    util.set("permissions_rwx", func)?;
    Ok(util)
}

/// Convert [Permission][8] to octal representation.
///
/// Type: function( [Permission][8] ) -> { number, number, number, number }
///
/// Example:
///
/// ```lua
/// xplr.util.permissions_octal({ user_read = true })
/// -- { 0, 4, 0, 0 }
///
/// xplr.util.permissions_octal(app.focused_node.permission)
/// -- { 0, 7, 5, 4 }
/// ```
pub fn permissions_octal(util: Table, lua: &Lua) -> Result<Table> {
    let func = lua.create_function(|lua, permission: Table| {
        let permissions: Permissions = lua.from_value(Value::Table(permission))?;
        let permissions: Octal = permissions.into();
        let permissions = lua::serialize(lua, &permissions).map_err(LuaError::custom)?;
        Ok(permissions)
    })?;
    util.set("permissions_octal", func)?;
    Ok(util)
}

///
/// [1]: https://xplr.dev/en/lua-function-calls#explorer-config
/// [2]: https://xplr.dev/en/lua-function-calls#node
/// [3]: https://xplr.dev/en/style
/// [4]: https://xplr.dev/en/layout
/// [5]: https://xplr.dev/en/lua-function-calls#node
/// [6]: https://xplr.dev/en/node-type
/// [7]: https://xplr.dev/en/node_types
/// [8]: https://xplr.dev/en/column-renderer#permission
///
pub(crate) fn create_table(lua: &Lua) -> Result<Table> {
    let mut util = lua.create_table()?;

    util = version(util, lua)?;
    util = debug(util, lua)?;
    util = clone(util, lua)?;
    util = exists(util, lua)?;
    util = is_dir(util, lua)?;
    util = is_file(util, lua)?;
    util = is_symlink(util, lua)?;
    util = is_absolute(util, lua)?;
    util = path_split(util, lua)?;
    util = node(util, lua)?;
    util = node_type(util, lua)?;
    util = dirname(util, lua)?;
    util = basename(util, lua)?;
    util = absolute(util, lua)?;
    util = relative_to(util, lua)?;
    util = shorten(util, lua)?;
    util = explore(util, lua)?;
    util = shell_execute(util, lua)?;
    util = shell_quote(util, lua)?;
    util = shell_escape(util, lua)?;
    util = from_json(util, lua)?;
    util = to_json(util, lua)?;
    util = from_yaml(util, lua)?;
    util = to_yaml(util, lua)?;
    util = lscolor(util, lua)?;
    util = paint(util, lua)?;
    util = style_mix(util, lua)?;
    util = textwrap(util, lua)?;
    util = layout_replace(util, lua)?;
    util = permissions_rwx(util, lua)?;
    util = permissions_octal(util, lua)?;

    Ok(util)
}

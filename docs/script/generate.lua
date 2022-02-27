-- Configuration documentation generator
--
-- TODO implement
--
-- This script generates configuration docs by parsing the comments and
-- properties from `src/init.lua`.
--
-- Usage:
--
-- ```
-- lua ./docs/script/generate.lua
-- ```
--
-- Or
--
-- ```
-- xplr -C ./docs/script/generate.lua --on-load Quit
-- ```

-- Generates ./docs/en/src/configuration.md
local function generate_configuration(lines) end

-- Generates ./docs/en/src/general-config.md
local function generate_general_config(lines) end

-- Generates ./docs/en/src/node_types.md
local function generate_node_types(lines) end

-- Generates ./docs/en/src/general-config.md
local function generate_layouts(lines) end

-- Generates ./docs/en/src/general-config.md
local function generate_modes(lines) end

local function main()
  local init = io.lines("./src/init.lua")

  generate_configuration(init)
  generate_general_config(init)
  generate_node_types(init)
  generate_layouts(init)
  generate_modes(init)
end

main()

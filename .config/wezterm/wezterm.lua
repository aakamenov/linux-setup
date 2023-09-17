-- Pull in the wezterm API
local wezterm = require 'wezterm'

-- This table will hold the configuration.
local config = {}

-- In newer versions of wezterm, use the config_builder which will
-- help provide clearer error messages
if wezterm.config_builder then
  config = wezterm.config_builder()
end

-- This is where you actually apply your config choices

config.font = wezterm.font('ShureTechMono Nerd Font', { weight = 'Regular', italic = false  })
config.font_size = 14.0
config.color_scheme = 'Ayu Mirage'
config.window_background_opacity = 0.3
config.default_prog = {"/bin/bash"}
config.warn_about_missing_glyphs = true

-- and finally, return the configuration to wezterm
return config

local Bar = {
  id = "test_bar",
  screen = 0,
  anchor = "top",
  axis = "horizontal", --  if set, "anchor" is ignored
  size = {
    width = "100%",
    height = 35,
  },
  position = { --  if set, "anchor" is ignored
    x = 0,
    y = 0
  },
  margin = {
    top = 0,
    right = 0,
    bottom = 0,
    left = 0
  },
  style = {
    background_color = "0x000000",
    font = "FontFace",
    text_color = "0xFFFFFF",
    font_size = 14,
    padding = {
      top = 0,
      right = 0,
      bottom = 0,
      left = 0
    }
    -- ...
  }

  -- type = "component",
  -- version = "1.0"
}

-- We should have an option to choose which set of styles will override
-- -> module.style > bar.style || bar.style > module.style ?


-- Import modules:
require "module1"
require "module2"

function Bar.Build()
  
end

function Bar.Render()
  
end

return Bar

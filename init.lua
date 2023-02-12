xplr.config.layouts.builtin.default = {
  Dynamic = "custom.render_layout",
}

xplr.fn.custom.render_layout = function(ctx)
  return {
    Table = {
      ui = { title = { format = ctx.app.pwd } },
      widths = {
        { Percentage = 50 },
        { Percentage = 50 },
      },
      body = {
        { "", "" },
        { "Layout height", tostring(ctx.layout_size.height) },
        { "Layout width", tostring(ctx.layout_size.width) },
        { "", "" },
        { "Screen height", tostring(ctx.screen_size.height) },
        { "Screen width", tostring(ctx.screen_size.width) },
      },
    },
  }
end

-- Nothing to see here. Please move on.
-- Or if you insist, see https://github.com/sayanarijit/xplr/issues/412

local xplr = xplr

xplr.__CACHE__ = { directory_nodes = {} }

function xplr.__CACHE__.set_directory_nodes(nodes)
  xplr.__CACHE__.directory_nodes = nodes
end

function xplr.__CACHE__.call(fun, arg)
  if arg.app and arg.app.directory_buffer then
    arg.app.directory_buffer.nodes = xplr.__CACHE__.directory_nodes
  elseif arg.directory_buffer then
    arg.directory_buffer.nodes = xplr.__CACHE__.directory_nodes
  end
  return fun(arg)
end

function xplr.__CACHE__.caller(fun)
  return function(arg)
    return xplr.__CACHE__.call(fun, arg)
  end
end

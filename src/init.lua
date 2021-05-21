version = "v0.10.0-beta.2"

xplr.fn.builtin.fmt_general_table_row_cols_0 = function(m)
    local r = ""
    if m.is_before_focus then
        r = r .. " -"
    else
        r = r .. "  "
    end

    r = r .. m.relative_index .. "│" .. m.index

    return r
end

xplr.fn.builtin.fmt_general_table_row_cols_1 = function(m)
    local r = m.tree .. m.prefix

    if m.meta.icon == nil then
        r = " " .. r
    end

    r = r .. m.relative_path

    if m.is_dir then
        r = r .. "/"
    end

    r = r .. m.suffix .. " "

    if m.is_symlink then
        r = r .. "-> "

        if m.is_broken then
            r = r .. "×"
        else
            r = r .. m.absolute_path
        end

        if m.symlink.is_dir then
            r = r .. "/"
        end
    end

    return r
end

xplr.fn.builtin.fmt_general_table_row_cols_2 = function(m)
    if not m.is_dir then
        return m.human_size
    else
        return ""
    end
end

xplr.fn.builtin.fmt_general_table_row_cols_3 = function(m)
    if m.is_symlink then
        return m.symlink.mime_essence
    else
        return m.mime_essence
    end
end

xplr.fn.custom.foo = function(a, b)
    return a + b
end

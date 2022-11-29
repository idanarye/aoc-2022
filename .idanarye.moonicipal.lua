local moonicipal = require'moonicipal'
local T = moonicipal.tasks_file()

local ffi = require'ffi'

local channelot = require'channelot'

function gen_all_implemented_days()
    local result = {}
    local pattern = vim.regex[=[\v^day\zs\d+\ze\.rs$]=]
    for name, typ in vim.fs.dir('src') do
        if typ == 'file' then
            local s, e = pattern:match_str(name)
            if s then
                table.insert(result, tonumber(name:sub(s + 1, e)))
            end
        end
    end
    table.sort(result)
    return result
end

function T:check()
    vim.cmd'Erun! cargo check -q'
end

function T:build()
    vim.cmd'Erun! cargo build -q'
end

function T:run()
    vim.cmd'botright new'
    channelot.terminal_job{'cargo', 'run', '--', '--day', vim.fn.max(gen_all_implemented_days())}
    vim.cmd.startinsert()
end

function T:act()
    vim.cmd'botright new'
    channelot.terminal_job{'cargo', 'run'}
    vim.cmd.startinsert()
end

function T:add_day()
    local day = tonumber(moonicipal.input{
        default = vim.fn.strftime('%d'),
        prompt = 'Day number: ',
    }) or moonicipal.abort('No number selected')
    vim.cmd('!copier -fd day=' .. day .. ' copy .copier/day-template .')
    do
        local lines = vim.fn.readfile('src/lib.rs')
        table.insert(lines, ('pub mod day%s;'):format(day))
        vim.fn.writefile(lines, 'src/lib.rs')
    end

    do
        local lines = vim.fn.readfile('src/main.rs')
        -- dump(lines)
        if lines[#lines] ~= '}' then
            error('Malformed main.rs')
        end
        table.insert(lines, #lines, ('    day%s : generator => part_1, part_2;'):format(day))
        dump(lines)
        vim.fn.writefile(lines, 'src/main.rs')
    end
    vim.cmd.checktime()
end

function T:demonstration_input()
    return self:cached_data_cell{}
end

function T:go()
    ffi.cdef'int fileno(struct FILE* stream);'
    local f = io.tmpfile()
    f:write(T:demonstration_input())
    f:write('\n')
    f:flush()
    local path = ('/proc/%s/fd/%s'):format(vim.fn.getpid(), ffi.C.fileno(f))
    vim.cmd'botright new'
    channelot.terminal_job{'cargo', 'run', '--', '--day', vim.fn.max(gen_all_implemented_days()), '--file', path}:wait()
    f:close()
end

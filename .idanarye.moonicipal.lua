local moonicipal = require'moonicipal'
local T = moonicipal.tasks_file()

local ffi = require'ffi'

local channelot = require'channelot'

T{alias=':2'}
function T:build_profile()
    local cc = self:cached_choice{key = vim.inspect}
    cc('dev')
    cc('release')
    return cc:select()
end

function T:query()
    dump({T:build_profile()})
end

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

function T:clean()
    vim.cmd'!cargo clean'
end

function T:check()
    vim.cmd('Erun! cargo check -q --profile ' .. T:build_profile())
end

function T:build()
    vim.cmd('Erun! cargo build -q --profile ' .. T:build_profile())
end

function T:run()
    vim.cmd'botright new'
    channelot.terminal_job{'cargo', 'run', '--profile', T:build_profile(), '--', '--day', vim.fn.max(gen_all_implemented_days())}
    vim.cmd.startinsert()
end

function T:act()
    vim.cmd'botright new'
    channelot.terminal_job{'cargo', 'run', '--profile', T:build_profile()}
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

T{alias=':3'}
function T:demonstration_input()
    return self:cached_data_cell{}
end

function T:go()
    local data = T:demonstration_input() or moonicipal.abort('No demonstration input')
    vim.cmd'botright new'
    vim.cmd.startinsert()
    local j = channelot.terminal_job{'cargo', 'run', '--profile', T:build_profile(), '--', '--day', vim.fn.max(gen_all_implemented_days()), '--stdin'}
    j:write(data)
    j:write('\n\4')
    j:wait()
end

function T:run_cargo_fmt()
    vim.cmd'!cargo fmt'
end

function T:clippy()
    vim.cmd'Erun! cargo clippy -q'
end

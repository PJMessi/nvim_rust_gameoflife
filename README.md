```lua

local M = {}

function M.setup()
    return {
        dir = '~/projects/personal/rust/gameoflife_rpc',
        lazy = false,
        config = function()
            require('gameoflife').setup()
        end,
    }
end

return M
```

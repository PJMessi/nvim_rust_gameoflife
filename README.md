I wrote a Game of Life plugin in Neovim in Rust just for fun. Why? I was learning Rust, and I love Neovim. The plugin keeps all the game logic and state inside Rust using nvim-rs, sending the evolving grid to Neovim each generation. I first tried nvim-oxi, which uses FFI to call Rust from Lua, but that forced me to manage state in Lua. Wanting full Rust control, I switched to JSON-RPC over stdin/stdout. But parsing JSON every frame in Lua could become costly—especially with a 100x100 grid, where a few hundred kilobytes of text have to be parsed each generation, which can become overhead for higher framerates (overkill for such a small plugin, I know 😂). So I switched to MessagePack (msgpack) RPC, which Neovim decodes natively, letting Lua get the grid as a table directly, no parsing needed, making updates smoother and faster.

```lua
local M = {}

function M.setup()
    return {
        dir = '~/projects/personal/rust/gameoflife_rpc', -- point this to your plugin location; remember to run `cargo build --release` beforehand to compile the Rust binary
        lazy = false,
        config = function()
            require('gameoflife').setup()
        end,
    }
end

return M
```

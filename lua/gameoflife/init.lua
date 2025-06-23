local M = {}

function M.setup()
  local plugin_path = debug.getinfo(1, "S").source:sub(2)
  local root = vim.fn.fnamemodify(plugin_path, ":h:h:h") -- go 3 dirs up: lua/gameoflife/init.lua -> plugin root

  local binary = root .. "/target/release/gameoflife_rpc"
  local log_path = "/tmp/gameoflife_rpc.log"
  local cmd = string.format("%s 2>%s", binary, log_path)

  local chan = vim.fn.jobstart({ "sh", "-c", cmd }, { rpc = true })
  print("Started gameoflife_rpc on channel:", chan)

  local buf = nil
  local frame_interval = 33

  local function update_grid()
    if not buf then
      vim.cmd("enew")
      buf = vim.api.nvim_get_current_buf()
    end

    local ok, result = pcall(vim.rpcrequest, chan, "gen_grid")
    if ok then
      vim.api.nvim_buf_set_lines(buf, 0, -1, false, result)
    else
      print("RPC error:", result)
    end

    vim.defer_fn(update_grid, frame_interval)
  end

  vim.defer_fn(update_grid, 1000)
end

return M

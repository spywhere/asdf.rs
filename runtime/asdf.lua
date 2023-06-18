print("Hello World from Lua")
local output, code = api.spawn {
  args = { "echo", "hello world", "from shell in lua" }
}

if code == 0 then
  print(output.stdout)
end

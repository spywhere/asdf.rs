local bin = plugin.file('bin/list-all')

if not bin then
  print('plugin corrupted')
  return
end

local code, output = api.spawn {
  command = bin
}

print(code)
if code == 0 then
  print(output.stdout)
end

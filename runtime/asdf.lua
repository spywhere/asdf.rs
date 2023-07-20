return {
  main = function ()
    local bin = plugin.file('bin/list-all')

    if not bin then
      print('plugin corrupted')
      return
    end

    local code, output = api.process.spawn {
      command = bin
    }

    if code == 0 then
      local versions = asdf.split_whitespace(output.stdout)
      print(table.concat(versions, '\n'))
    else
      print('Error while fetching available versions')
    end
  end
}

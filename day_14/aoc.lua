read_file = function (path)
    local file = io.open(path, "rb")
    if not file then return nil end

    local lines = {}

    for line in io.lines(path) do
        local words = {}
        for word in line:gmatch("-?%d+") do
            table.insert(words, tonumber(word))
        end
        table.insert(lines, words)
    end

    file:close()
    return lines
end

local filename = arg[1] or "example"
local width = arg[1] and 101 or 11
local height = arg[1] and 103 or 7

local input = read_file(filename)

local quadrants = {0, 0, 0, 0}
for _, v in ipairs(input) do
    local x = (v[1] + 100 * v[3]) % width
    local y = (v[2] + 100 * v[4]) % height

    if x < width // 2 and y < height // 2 then
        quadrants[1] = quadrants[1] + 1
    elseif x > width // 2 and y < height // 2 then
        quadrants[2] = quadrants[2] + 1
    elseif x < width // 2 and y > height // 2 then
        quadrants[3] = quadrants[3] + 1
    elseif x > width // 2 and y > height // 2 then
        quadrants[4] = quadrants[4] + 1
    end
end

print("Part 1: ", quadrants[1] * quadrants[2] * quadrants[3] * quadrants[4])

local s = 0
local f = 1
local g = {}

for i, v in ipairs(input) do
    input[i][1] = (input[i][1] + s * input[i][3]) % width
    input[i][2] = (input[i][2] + s * input[i][4]) % height

    g[input[i][1] + width * input[i][2]] = "#"
end

while true do
    s = s + f
    print("Second: ", s)
    for i=1,width*height do
        g[i] = "."
    end

    for i, v in ipairs(input) do
        input[i][1] = (input[i][1] + f * input[i][3]) % width
        input[i][2] = (input[i][2] + f * input[i][4]) % height

        g[input[i][1] + width * input[i][2]] = "#"
    end

    for i, v in ipairs(g) do
        io.write(v)
        if i % width == 0 then
            print()
        end
    end

    io.read()
    os.execute("clear")
end

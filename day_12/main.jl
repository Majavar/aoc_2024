#!/usr/bin/env julia
filename = length(ARGS) > 0 ? ARGS[1] : "example"

input = open(filename, "r") do f
    map(
        l -> "*" * l * "*",
        filter!(
            x -> x != "",
            split(read(f, String), '\n'))
    )
end

(width, height) = (length(input[1]), length(input) + 2)

insert!(input, 1, "*"^width)
push!(input, "*"^width)


regions = fill(0, (height, width))
id = Iterators.Stateful(Iterators.countfrom(1))

for i in 2:width-1
    for j in 2:height-1
        if regions[i, j] == 0
            region = first(id)
            value = input[i][j]
            stack = Vector{Tuple{Int, Int}}()

            push!(stack, (i, j))
            while !isempty(stack)
                (x, y) = pop!(stack)
                if regions[x, y] == 0
                    regions[x, y] = region

                    foreach(
                        p -> begin
                            if input[p[1]][p[2]] == value
                                push!(stack, (p[1], p[2]))
                            end
                        end,
                        [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                    )
                end
            end
        end
    end
end

counter = Dict()
for i in 2:width-1
    for j in 2:height-1
        if !haskey(counter, regions[i, j])
            counter[regions[i, j]] = [0, 0]
        end

        counter[regions[i, j]][1] += 1
        regions[i - 1, j] != regions[i, j] ? counter[regions[i, j]][2] += 1 : nothing
        regions[i + 1, j] != regions[i, j] ? counter[regions[i, j]][2] += 1 : nothing
        regions[i, j - 1] != regions[i, j] ? counter[regions[i, j]][2] += 1 : nothing
        regions[i, j + 1] != regions[i, j] ? counter[regions[i, j]][2] += 1 : nothing

    end
end

part1 = sum(map(x -> x[1] * x[2], values(counter)))
print("Part 1: $part1\n")

counter = Dict()
for i in 2:width-1
    for j in 2:height-1
        if !haskey(counter, regions[i, j])
            counter[regions[i, j]] = [0, 0]
        end

        counter[regions[i, j]][1] += 1
        regions[i - 1, j] != regions[i, j] && !(regions[i, j - 1] == regions[i, j] && regions[i - 1, j - 1] != regions[i, j]) ? counter[regions[i, j]][2] += 1 : nothing
        regions[i + 1, j] != regions[i, j] && !(regions[i, j - 1] == regions[i, j] && regions[i + 1, j - 1] != regions[i, j]) ? counter[regions[i, j]][2] += 1 : nothing
        regions[i, j - 1] != regions[i, j] && !(regions[i - 1, j] == regions[i, j] && regions[i - 1, j - 1] != regions[i, j]) ? counter[regions[i, j]][2] += 1 : nothing
        regions[i, j + 1] != regions[i, j] && !(regions[i - 1, j] == regions[i, j] && regions[i - 1, j + 1] != regions[i, j]) ? counter[regions[i, j]][2] += 1 : nothing
    end
end

part2 = sum(map(x -> x[1] * x[2], values(counter)))
print("Part 2: $part2\n")

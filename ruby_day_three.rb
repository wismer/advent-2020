
lines = File.readlines("../day3.txt").map(&:chomp)

def part_one(ls)
  trees = 0
  ls.each_with_index do |line, index|
    if line[(3 * index) % line.length] == '#'
      trees += 1
    end
  end
  trees
end

puts("Part One #{part_one(lines)}")


def part_two(ls)
  trees = 0
  paths = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]
  # paths = [[3, 1]]

  paths.map! do |path|
    trees = 0
    ls.each_with_index do |line, index|
      next if index % path[1] != 0
      if line[(index / path[1] * path[0]) % line.length] == '#'
        trees += 1
      end
    end
    trees
  end

  paths.inject(:*)
end


puts("Part two #{part_two(lines)}")

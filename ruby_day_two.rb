require 'pry'
def get_info(line)
  raw_min, raw_max, requirement, password = line.match(/(\d+)\-(\d+)\s(\w)\:\s(\w+)/).captures
  min, max = raw_min.to_i, raw_max.to_i
  count = password.count(requirement)
  
  {
    min: min,
    max: max,
    requirement: requirement,
    password: password,
    count: count
  }
end

file_contents = File.readlines('../day2.txt').map(&:chomp)

def part_one(lines)
  tally = 0
  lines.each do |line|
    info = get_info(line)
    if (info[:min]..info[:max]).include?(info[:count])
      tally += 1
    end
  end
  puts("PART ONE TOTAL: #{tally}")
end

def part_two(lines)
  tally = 0

  lines.each do |line|
    info = get_info(line)

    min, max, password, req = info[:min], info[:max], info[:password], info[:requirement]

    pattern = [password[min-1] == req, password[max-1] == req]
    if pattern.count(true) == 1
      tally += 1
      puts(info)
    end
  end

  puts("PART TWO TOTAL: #{tally}")
end

part_one(file_contents)

part_two(file_contents)
require 'pry'
lines = File.readlines("../day4.txt").map(&:chomp)

raw_passports = []
chunks = []

def parse_passport_chunks(parts)
  fields = {}
  parts.each do |line|
    line.split(' ').each do |field|
      key, value = field.split(':')
      fields[key] = value
    end
  end

  fields
end

EYE_COLORS = 'amb blu brn gry grn hzl oth'.split(' ')

def is_valid?(passport)

  valid_keys = [
    {
      key: 'byr',
      validator: lambda { |value| (1920..2002).include?(value.to_i) }
    }, 
    {
      key: 'iyr',
      validator: lambda { |value| (2010..2020).include?(value.to_i) } 
    },
    {
      key: 'eyr',
      validator: lambda { |value| (2020..2030).include?(value.to_i) }
    },
    {
      key: 'hgt',
      validator: lambda { |value| value.include?('cm') ? (150..193).include?(value.to_i) : (59..76).include?(value.to_i) },
    },
    {
      key: 'hcl',
      validator: lambda { |value| value.match?(/\#[a-f0-9]{6}/) },
    },
    {
      key: 'ecl',
      validator: lambda { |value| EYE_COLORS.include?(value) }
    },
    {
      key:'pid',
      validator: lambda { |value| value.match?(/\d{9}/) }
    }
  ]
  if valid_keys.all? { |k| passport[k[:key]] != nil && k[:validator].call(passport[k[:key]]) }
    return true
  end

  return false
end

valid_count = 0
lines.each_with_index do |line, index|
  if line.length > 0
    chunks << line
  else
    passport = parse_passport_chunks(chunks)

    if is_valid?(passport)
      raw_passports << passport
    end
    chunks = []
  end
  # puts line
end

last_pp = parse_passport_chunks(chunks)
raw_passports << last_pp if is_valid?(last_pp)

puts raw_passports.length
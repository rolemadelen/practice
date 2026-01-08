# https://www.acmicpc.net/problem/10773

n = gets.chomp.to_i
stk = []

n.times do
    val = gets.chomp.to_i
    case val
    when 0
        stk.shift
    else
        stk.unshift val
    end
end

puts stk.sum
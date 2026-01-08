tc = gets.to_i

tc.times do
    stk = []
    p = gets.chomp

    p.each_char do |ch|
        if ch == '('
            stk.push ch
        else
            if stk.size == 0 || stk[-1] == ')'
                stk.push ch
                break
            end
            stk.pop
        end
    end
    if stk.size == 0
        puts "YES"
    else
        puts "NO"
    end
end
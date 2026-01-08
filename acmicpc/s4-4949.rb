loop do
    s = gets.chomp
    break if s.size == 1 && s == "."
    s2 = s.gsub(/[a-zA-Z0-9 .]/, '')

    stk = []
    s2.each_char do |ch|
        if ch == '(' || ch == '['
            stk.push ch
        else
            last_ch = stk[-1]
            if ch == ')' && last_ch == '('
                stk.pop
            elsif ch == ']' && last_ch == '['
                stk.pop
            else
                stk.push ch
                break
            end
        end
    end
    
    if stk.size == 0
        puts "yes"
    else
        puts "no"
    end
end
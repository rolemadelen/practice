class StackArray
    def initialize
        @top = []
    end

    def push(data)
        @top.unshift(data)
    end

    def pop
        @top.shift
    end

    def size
        @top.size
    end

    def peek
        @top.first
    end

    def empty?
        return @top.size == 0
    end

    def each 
        idx = 0
        while idx < @top.size
            yield @top[idx]
            idx += 1
        end
    end

    def to_s
        if @top.size != 0
            each { |x| print "#{x} "}
            puts
        end
    end
    
    alias_method :length, :size
end

stack = StackArray.new
stack.push(1)
stack.push(3)
stack.push(5)
stack.to_s

puts stack.peek

stack.pop
stack.pop
stack.pop
stack.pop
stack.to_s
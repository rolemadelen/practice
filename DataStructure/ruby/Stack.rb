class Stack
    private class Node
        attr_accessor :data, :next
        def initialize(data)
            @data = data
            @next = nil
        end
    end

    def initialize
        @top = nil
        @size = 0
    end

    def push(data)
        node = Node.new(data)
        if @top == nil
            @top = node
        else
            node.next = @top
            @top = node
        end

        @size += 1
    end

    def pop
        if @top != nil
            ret = @top.data
            @top = @top.next
            @size -= 1
        end

        ret
    end

    def size
        @size
    end

    def peek
        return @top.data if @top != nil
    end

    def empty?
        return @size == 0
    end

    def each 
        curr = @top
        while curr
            yield curr
            curr = curr.next
        end
    end

    def to_s
        if @size != 0
            each { |node| print "#{node.data} "}
            puts
        end
    end
    
    alias_method :length, :size
end

stack = Stack.new
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
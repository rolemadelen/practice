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
        ret = -1
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

    def empty?
        if @size == 0
            return 1
        else
            return 0
        end
    end

    def peek
        ret = -1
        if @top != nil
            ret = @top.data
        end

        ret
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

n = gets.to_i
n.times do
    cmd = gets

    case cmd[0].to_i
    when 1
        val = cmd.split(' ')[1]
        stack.push(val)
    when 2
        puts stack.pop
    when 3
        puts stack.size
    when 4
        puts stack.empty?
    when 5
        puts stack.peek
    else
        puts "error"
    end
end
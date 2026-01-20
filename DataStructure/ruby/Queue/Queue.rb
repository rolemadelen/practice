class Queue
    private class Node
        attr_accessor :next, :data

        def initialize(data)
            @data = data
            @next = nil
        end
    end

    def initialize
        @head = nil
        @tail = nil
        @size = 0
    end

    def enqueue(data)
        new_node = Node.new(data)

        if @head == nil
            @head = new_node
            @tail = new_node
            @head.next = @tail
        else
            @tail.next = new_node
            @tail = new_node
        end

        @size += 1
    end

    def dequeue
        if @head != nil
            @head = @head.next
            @size -= 1
        end

        if @size == 0
            @head = nil
            @tail = nil
        end
    end

    def front
        self.empty? ? nil : @head.data

    end
    alias_method :first, :front

    def rear
        self.empty? ? nil : @tail.data
    end
    alias_method :last, :rear

    def empty?
        @size == 0
    end

    def size
        @size
    end
    alias_method :length, :size

    def to_s
        curr = @head

        while curr != nil
            print "#{curr.data} "
            curr = curr.next
        end
        puts
    end
end

queue = Queue.new

queue.enqueue(1)
queue.enqueue(2)
queue.enqueue(3)
queue.to_s

puts "empty? #{queue.empty?}"
puts "size #{queue.size}"

puts "front: #{queue.front}"
puts "first: #{queue.first}"

puts "rear: #{queue.rear}"
puts "last: #{queue.last}"

while !queue.empty? do
    queue.dequeue

    puts "front: #{queue.front}"
    puts "first: #{queue.first}"

    puts "rear: #{queue.rear}"
    puts "last: #{queue.last}"
    puts "size: #{queue.size}"
    puts "empty? #{queue.empty?}"
    puts
end

queue.enqueue(1)
queue.enqueue(3)
queue.enqueue(5)
queue.to_s

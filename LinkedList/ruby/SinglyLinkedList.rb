class Node
    attr_accessor :data, :next

    def initialize(data)
        @data = data
        @next = nil
    end
end

class SinglyLinkedList
    def initialize
        @head = nil
        @tail = nil
        @size = 0
    end

    def push_back(data)
        node = Node.new(data)

        if @size == 0
            @head = node
            @tail = node
            @head.next = @tail
        else
            @tail.next = node
            @tail = node
        end

        @size += 1
    end

    def push_front(data)
        node = Node.new(data)

        if @size == 0
            @head = node
            @tail = node
            @head.next = @tail
        else
            node.next = @head
            @head = node
        end

        @size += 1
    end

    def insert(pos, data)
        if pos == 0
            push_front(data)
        elsif pos == @size
            push_back(data)
        else
            if pos > @size || pos < 0
                puts "invalid position"
                return nil
            end
            temp_node = get_nth_node(pos-2)
            new_node = Node.new(data)

            new_node.next = temp_node.next
            temp_node.next = new_node
            @size += 1
        end
    end

    def pop_front
        if @size == 0
            puts "list is empty."
            return nil
        elsif @size == 1
            @head = @tail = nil
        else
            pop_node = @head
            @head = @head.next
            pop_node = pop_node.next = nil
        end

        @size -= 1
    end

    def pop_back
        if @size == 0
            puts "list is empty."
            return nil
        elsif @size == 1
            @head = @tail = nil
        else
            before_tail = get_nth_node(@size-2)
            before_tail.next = nil
            @tail = before_tail
        end

        @size -= 1
    end

    def delete_at(pos)
        if pos == 0
            pop_front
        elsif pos == @size-1
            pop_back
        else
            if pos > @size || pos < 0
                puts "invalid position"
                return nil
            end
            temp_node = get_nth_node(pos-1)
            temp_node.next = temp_node.next.next
            @size -= 1
        end
    end

    def delete_key(data)
        return nil if has?(data) == false
        
        if @head.data == data
            pop_front
        elsif @tail.data == data
            pop_back
        else
            curr = @head
            pos = 0
            while curr.next.data != data
                curr = curr.next
                pos += 1
            end

            delete_at(pos+1)
        end
    end

    def has?(data)
        curr = @head
        while curr != nil
            return true if curr.data == data
            curr = curr.next
        end

        false
    end

    def empty?
        @size == 0
    end

    def size
        @size
    end
    alias_method :length, :size

    def display
        if @head == nil
            puts "list is empty"
            return nil
        end

        curr = @head
        while curr.next != nil
            print "#{curr.data} "
            curr = curr.next
        end
        puts curr.data
    end

    def reverse
    end

    private def get_nth_node(pos)
        curr = @head
        pos.times { curr = curr.next }
        curr
    end
end

list = SinglyLinkedList.new
puts "List is empty? #{list.empty?}"

list.push_back(4)
list.push_back(5)
list.push_back(6)
list.push_back(7)
list.display

list.push_front(3)
list.push_front(2)
list.push_front(1)
list.display

puts "List has 1? #{list.has?(1)}"
puts "List has 10? #{list.has?(10)}"
puts "List is empty? #{list.empty?}"

puts "list size: #{list.size}"
puts "list size: #{list.length}"

puts

list.pop_front
list.display
list.pop_back
list.display

list.push_front(1)
list.push_back(12)
list.display

list.insert(0, 0)
list.insert(8, 17)
list.display

list.delete_at(1)
list.display

list.delete_at(6)
list.display

list.delete_at(0)
list.display

list.push_front(1)
list.display

list.pop_back
list.display

list.delete_key(2)
list.delete_key(4)
list.display

list.delete_key(1)
list.display
list.delete_key(3)
list.display
list.delete_key(5)
list.display
list.delete_key(7)
list.display

puts

list.pop_back
list.display
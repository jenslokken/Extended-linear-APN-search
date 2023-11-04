import sys

def find_cycles(l2):
    visited = [False] * len(l2)
    total = 1
    cycles = []
    cycle_sizes = []

    first = 0 # initialize the first element of the cycle to to be the first element of l2
    cycle = [0]
    visited[0] = True

    while total < len(l2):
        current = l2[first]
        # if current element is equal to the first element we have looped
        while current != first:
            visited[current] = True
            cycle.append(current)
            total += 1
            current = l2[current]

        else:
            cycles.append(cycle)
            cycle_sizes.append(len(cycle))
            cycle = list()
            # find the next unvisited element
            for i in range(len(l2)):
                if not visited[i]:
                    first = i
                    cycle.append(first)
                    visited[first] = True
                    total += 1
                    break
        
        if total == len(l2) and len(cycle) > 0:
            cycles.append(cycle)
            cycle_sizes.append(len(cycle))
    return cycles[1:], cycle_sizes[1:]

def read_file(filename):
    content = []
    with open(filename, "r") as f:
        labels = f.readline().strip().split(";")
        for line in f:
            line = line.strip().split(";")
            line = [line[0]] + list(map(lambda x: list(map(int, x.split())), line[1:]))
            content.append(line)
    return content, labels

def write_file(filename, content):
    with open(filename, 'a') as f:
        for line in content:
            f.write(line)

def main():
    if len(sys.argv) != 3:
        print("Usage: " + sys.argv[0] + " InputFile OutputFile")
        return
    content, labels = read_file(sys.argv[1])
    output = []
    for line in content:
        cycles, cycle_sizes = find_cycles(line[2])
        output.append(f"\n{line[0]};{' '.join(map(str, line[2]))};{len(cycles)};{cycle_sizes};{';'.join(map(lambda i: ' '.join(map(str, i)), cycles))}")
    write_file(sys.argv[2], output)

main()      

f = open("input2.txt", "r")

InitalMemory = [int(s) for s in f.read().split(',')]

for Noun in range(100):
    for Verb in range(100):
        memory = InitalMemory.copy() 
        memory[1] = Noun
        memory[2] = Verb
        
        PC = 0
        while memory[PC] != 99:
            Op1 = memory[PC + 1]
            Op2 = memory[PC + 2]
            Dest = memory[PC + 3]
            if memory[PC] == 1:
                memory[Dest] = memory[Op1] + memory[Op2]
            elif memory[PC] == 2:
                memory[Dest] = memory[Op1] * memory[Op2]
            else:
                break
            PC += 4
        
        if (memory[0] == 19690720):
            print(memory)
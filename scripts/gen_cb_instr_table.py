class Switch(dict):
    def __getitem__(self, item):
        for key in self.keys():                 # iterate over the intervals
            if item in key:                     # if the argument is in that interval
                return super().__getitem__(key) # return its associated value
        raise KeyError(item)  

registers = [ "B", "C", "D", "E", "H", "L", "HL", "A" ]

def gen_range(i):
    return lambda: range(i*8, (i+1)*8)

mnemonics = Switch({
    gen_range(0)(): "RLC",
    gen_range(1)(): "RRC",
    gen_range(2)(): "RL",
    gen_range(3)(): "RR",
    gen_range(4)(): "SLA",
    gen_range(5)(): "SRA",
    gen_range(6)(): "SWAP",
    gen_range(7)(): "SRL",
    gen_range(8)(): "BIT0",
    gen_range(9)(): "BIT1",
    gen_range(10)(): "BIT2",
    gen_range(11)(): "BIT3",
    gen_range(12)(): "BIT4",
    gen_range(13)(): "BIT5",
    gen_range(14)(): "BIT6",
    gen_range(15)(): "BIT7",
    gen_range(16)(): "RES0",
    gen_range(17)(): "RES1",
    gen_range(18)(): "RES2",
    gen_range(19)(): "RES3",
    gen_range(20)(): "RES4",
    gen_range(21)(): "RES5",
    gen_range(22)(): "RES6",
    gen_range(23)(): "RES7",
    gen_range(24)(): "SET0",
    gen_range(25)(): "SET1",
    gen_range(26)(): "SET2",
    gen_range(27)(): "SET3",
    gen_range(28)(): "SET4",
    gen_range(29)(): "SET5",
    gen_range(30)(): "SET6",
    gen_range(31)(): "SET7",
})

def print_instr_str(ii, mnemonic):
    cycles = 16 if len(registers[ii%8]) == 2 else 8
    print("/*"+hex(ii)+"*/ Instruction{encoding:Type::CB,mnemonic:\""+mnemonic+"_"+registers[ii%8]+"\",cycles:"+str(cycles)+",length:2},")

for ii in range(256):
    print_instr_str(ii, mnemonics[ii])
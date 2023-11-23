from labels import Labels
from Functions import *
import r2pipe, os, json, time


Labels.setup()
# print(Labels.get_label(0x51a5dc49e))


def scan_for_adrp_val(disassembly, offset):
    start_idx = next((idx for (idx, d) in enumerate(disassembly) if d["offset"] == offset))
    res = 0
    for x in range(start_idx, len(disassembly)):
        instr = disassembly[x]
        split = [x.strip() for x in instr["opcode"].split(' ', 1)]
        opcode = split[0]
        if opcode == "adrp":
            data = [x.strip() for x in split[1].split(',')]
            val = int(data[1][2:], 16)
            res = val
            instr = disassembly[x + 1]
            split = [x.strip() for x in instr["opcode"].split(' ', 1)]
            opcode = split[0]
            if opcode == "add":
                data = [x.strip() for x in split[1].split(',')]
                val = int(data[2][2:], 16)
                res += val
                return res
            
filepath = "../elfs/lua2cpp_ryu.elf"
filename = os.path.basename(filepath)

r2 = r2pipe.open(filepath, flags=["-e bin.cache=true", "-2"])
r2.cmd('e anal.vars = false')
r2.cmd('e anal.bb.maxsize = 0x10000')
r2.cmd('e anal.depth = 128')
Labels.setup_imports(r2.cmdj('iij'))


# vaddr = 0
# for x in r2.cmdj("isj"):
#     if "demname" in x:
#         if "create_agent_" in x["demname"] and "_status_" in x["demname"]:
#             print("Found %s" % x["demname"])
#             vaddr = x["vaddr"]

# if vaddr == 0:
#     exit()


# create_agent_fighter_status = r2.cmdj('s {0};aF;pdfj'.format(hex(vaddr)))

# articles_to_offset = {}
# articles_to_vtable = {}
# registers = {}

# with open("tmp.json", "w+") as f:
#     json.dump(create_agent_fighter_status["ops"], f, indent=4)


# for z in range(len(create_agent_fighter_status["ops"])):
#     instr = create_agent_fighter_status["ops"][z]
#     split = [x.strip() for x in instr["opcode"].split(' ', 1)]
#     opcode = split[0]

#     if opcode == "mov":
#         data = [x.strip() for x in split[1].split(',')]
#         val = instr["opcode"].split(' ')[2].strip()
#         mul = 1
#         if val[0] == "-":
#             mul = -1
#             val = val[1:]
#         if val[0] == 'v':
#             continue
#         if val.startswith("0x"):
#             val = int(val[2:], 16) * mul
#         elif val[0] == "x" or val[0] == "w":
#             if val in registers:
#                 val = registers[val]
#             else:
#                 val = 0
#         else:
#             val = int(val) * mul
#         registers[data[0]] = val
#     elif opcode == "movk":
#         data = [x.strip() for x in split[1].split(',')]
#         if data[1].startswith("0x"):
#             val = int(data[1][2:], 16)
#         else:
#             val = int(data[1])
#         if data[2] == "lsl 16":
#             registers[data[0]] += (val << 16)
#         elif data[2] == "lsl 32":
#             registers[data[0]] += (val << 32)
#     elif opcode == "b.ne":
#         articles_to_offset[registers['x9']] = instr["offset"] + 4
#     elif opcode == "b.eq":
#         articles_to_offset[registers['x9']] = instr["jump"]

# for article in articles_to_offset:
#     articles_to_vtable[article] = scan_for_adrp_val(create_agent_fighter_status["ops"], articles_to_offset[article])

# for x in articles_to_vtable:
#     print(Labels.get_label(x), articles_to_vtable[x])

articles_to_status_script = {
    0x56031f3df: 0x640 # Article: Chrom
}

# with open("special_n_end_3_main.json", "w+") as f:
#     json.dump(r2.cmdj('s {0};aF;pdfj'.format(hex(0x1b500)))["ops"], f, indent=4)

# with open("special_n_end_3_main_function_call.json", "w+") as f:
#     json.dump(r2.cmdj('s {0};aF;pdfj'.format(hex(0x22c00)))["ops"], f, indent=4)

# with open("random.json", "w+") as f:
#     json.dump(r2.cmdj('s {0};aF;pdfj'.format(hex(0x148d0)))["ops"], f, indent=4)


# with open("random.json", "w+") as f:
#     json.dump(r2.cmdj('s {0};aF;pdfj'.format(hex(0x176e0)))["ops"], f, indent=4)



# setup_status_fun = SetStatusFunction(r2, Registers(), r2.cmdj('s {0};aF;pdfj'.format(hex(0x640)))["ops"], 'x0')
# setup_status_fun.run()


# test_func = StatusFunction(r2, Registers(), r2.cmdj('s {0};aF;pdfj'.format(hex(0x268c0)))["ops"], 'x0')
# test_func.run()
# print('\n\n'.join(test_func.output))
# open("special_n_pre.rs", "w+").write('\n\n'.join(test_func.output))


# special_n_end_max_main = StatusFunction(r2, Registers(), r2.cmdj('s {0};aF;pdfj'.format(hex(0x176e0)))["ops"], ['x0'])
# special_n_end_max_main.run()

# print('\n'.join(special_n_end_max_main.output))

ryu_turn_auto_main = StatusFunction(r2, Registers(), r2.cmdj('s {0};aF;pdfj'.format(hex(0x2b0c0)))["ops"], ['x0'])
ryu_turn_auto_main.run()

print('\n'.join(ryu_turn_auto_main.output))
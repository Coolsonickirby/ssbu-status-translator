import json, struct, copy
from labels import Labels
Labels.setup()

class Registers:
    def __init__(self) -> None:
        self.registers = {
            "x": [0] * 31,
            "w": [0] * 31,
            "s": [0] * 31
        }
    
    def get_register(self, reg):
        type = reg[0]
        idx = reg[1:]
        val = self.registers[type][int(idx)]
        if type == "s":
            try:
                val = float(val)
            except:
                pass
        else:
            try:
                val = int(val)
            except:
                pass
        return val
    
    def set_register(self, reg, value):
        type = reg[0]
        idx = reg[1:]
        self.registers[type][int(idx)] = value

class StatusFunction:
    tmp = 0

    def __init__(self, 
                 r2,
                 regs,
                 instructions,
                 fighter_register,
                 start_instruction = 0,
                 tmp = 0,
                 indent_level=0,
                 depth=0,
                 offsets_at_b_or_tbz=[],
                 stop_at_idx=None
                 ) -> None:
        self.output = []
        self.args = []
        self.r2 = r2
        self.Registers = regs
        self.current_instruction = 0
        self.instructions = instructions
        self.stack = []
        self.sp = 0
        self.fighter_register = fighter_register
        self.tmp = tmp
        self.jump_back_to = []
        self.start_instruction = start_instruction
        self.indent_level = indent_level
        self.updateIndent()
        self.depth = depth
        self.offsets_at_b_or_tbz = offsets_at_b_or_tbz
        self.stop_at_idx = stop_at_idx
        if len(self.offsets_at_b_or_tbz) == 0:
            for x in range(len(self.instructions)):
                if self.instructions[x]["opcode"].startswith("b "):
                    if self.instructions[x]["jump"] not in self.offsets_at_b_or_tbz:
                        self.offsets_at_b_or_tbz.append(self.instructions[x]["jump"])

    def parse_entry(self, entry):
        if "sp" in entry:
            entry = self.sp
        elif "." in entry:
            entry = float(entry)
        elif entry.startswith("0x"):
            entry = int(entry[2:], 16)
        elif entry == "wzr":
            entry = 0
        elif entry.startswith("x") or entry.startswith("w") or entry.startswith("s"):
            entry = self.Registers.get_register(entry)
        else:
            entry = int(entry) if entry.isnumeric() else entry
        return entry

    def parse_movz(self, instr=None):
        if instr == None:
            instr = self.instructions[self.current_instruction]
        else:
            instr = {"opcode": instr}

        if "mov v" in instr["opcode"]:
            return
            
        register = instr["opcode"].split(' ')[1].strip()[:-1]
        value = instr["opcode"].split(' ')[2].strip()

        if register in self.fighter_register:
            self.fighter_register.remove(register)

        if "sp" in value:
            return


        value = self.parse_entry(value)
        if register[0] == "w" and (instr["opcode"].split(' ')[2].strip() != "wzr" and isinstance(value, str) == False):
            register = "x" + register[1:]

        if instr["opcode"].split(' ')[2].strip() in self.fighter_register: # Change the fighter register if its moved (for w/e reason)
            self.fighter_register.append(register)

        self.Registers.set_register(register, value)

    def parse_adrp(self):
        instr = self.instructions[self.current_instruction]
        register = instr["opcode"].split(' ')[1].strip()[:-1]
        value = int(instr["opcode"].split(' ')[2].strip()[2:], 16)
        self.Registers.set_register(register, value)

    def parse_ldr(self):
        instr = self.instructions[self.current_instruction]
        
        if "sp" in instr["opcode"]:
            return
        
        dest_register = instr["opcode"].split(' ')[1].strip()[:-1]

        (src_register, offset) = instr["opcode"].split(' ', 2)[-1].split(',')
        src_register = src_register[1:].strip()

        offset = offset.strip()[:-1]

        if offset.startswith("0x"):
            offset = int(offset.strip()[2:], 16)
        elif offset.startswith("x"):
            offset = self.Registers.get_register(offset)


        type = dest_register[0]
        
        if type == "x":
            if src_register in self.fighter_register:
                self.Registers.set_register(dest_register, Labels.get_fighter_offset(offset))
                self.args = ["%s" % (self.Registers.get_register(dest_register))] + self.args
            else:
                data = self.r2.cmd('s {0};pf {1}'.format(self.Registers.get_register(src_register) + offset, 'i'))
                data = int(data[data.index('=') + 1:].strip())
                self.Registers.set_register(dest_register, data)
        elif type == "w":
            idx = int((offset / 4)) + 1
            if idx in Labels.constants:
                self.Registers.set_register(dest_register, Labels.constants[idx])
            else:
                self.Registers.set_register(dest_register, 0)

        # self.Registers.set_register(register, value)

    def parse_add(self):
        instr = self.instructions[self.current_instruction]["opcode"]
        registers = [y.strip()[:-1] if y.endswith(",") else y.strip() for y in instr.split(' ')[1:]]
        
        dest_register = registers[0]
        src_1 = registers[1]
        src_2 = registers[2]

        if "sp" in src_1 or "sp" in src_2:
            return

        if "sp" in src_1:
            src_1 = self.sp
        elif src_1.startswith("0x"):
            src_1 = int(src_1[2:], 16)
        else:
            src_1 = self.Registers.get_register(src_1)

        if "sp" in src_2:
            src_2 = self.sp
        elif src_2.startswith("0x"):
            src_2 = int(src_2[2:], 16)
        else:
            src_2 = self.Registers.get_register(src_2)
        
        if dest_register == "sp":
            return
        else:
            offset = 0
            if registers[1] in self.fighter_register:
                offset = src_2
            elif registers[2] in self.fighter_register:
                offset = src_1
            
            if offset == 0:
                self.Registers.set_register(dest_register, src_1 + src_2)
            else:
                self.Registers.set_register(dest_register, Labels.get_fighter_offset(offset))

    def return_register_is_used(self, reg_num, start_inst_idx, inst_count):
        table = ["w%s" % reg_num, "x%s" % reg_num, "s%s" % reg_num]
        for x in range(start_inst_idx, start_inst_idx + inst_count):
            if x > len(self.instructions):
                return False
            
            instr = self.instructions[x]["opcode"]
            if instr.startswith("bl"): # Overwrites the current instruction --- Uhhh??? Sometimes they get passed directly lol
                # Update this function to keep track of register being moved around ig
                if "L2CValue" in self.instructions[x]["disasm"]:
                    return True
                else:
                    return False
            if instr.startswith("mov"):
                registers = [y.strip()[:-1] if y.endswith(",") else y.strip() for y in instr.split(' ')[1:]]
                if registers[1] in table:
                    return True
            elif instr.startswith("and"):
                registers = [y.strip()[:-1] if y.endswith(",") else y.strip() for y in instr.split(' ')[1:]]
                if registers[1] in table or registers[2] in table:
                    return True
                


    def parse_orr(self):
        instr = self.instructions[self.current_instruction]["opcode"]
        registers = [y.strip()[:-1] if y.endswith(",") else y.strip() for y in instr.split(' ')[1:]]
        
        dest_register = registers[0]
        value_1 = self.parse_entry(registers[1])
        value_2 = self.parse_entry(registers[2])

        if isinstance(self.Registers.get_register(dest_register), list) == False:
            res = []
            if isinstance(value_1, list):
                res = res + value_1
            else:
                res = res + [value_1]
            if isinstance(value_2, list):
                res = res + value_2
            else:
                res = res + [value_2]
            self.Registers.set_register(dest_register, res)
        else:
            res = self.Registers.get_register(dest_register)
            if isinstance(value_1, list):
                res = res + value_1
            else:
                res = res + [value_1]
            if isinstance(value_2, list):
                res = res + value_2
            else:
                res = res + [value_2]
            self.Registers.set_register(dest_register, res)

    def parse_and(self):
        instr = self.instructions[self.current_instruction]["opcode"]
        registers = [y.strip()[:-1] if y.endswith(',') else y.strip() for y in instr.split(' ')[1:]]

        destination_register = registers[0]
        src_1 = self.parse_entry(registers[1])
        src_2 = self.parse_entry(registers[2])

        if destination_register[0] == "w":
            destination_register = "x" + destination_register[1:]

        if src_1 == "True" or src_1 == "False":
            src_1 = 1 if src_1 == "True" else 0

        if src_2 == "True" or src_2 == "False":
            src_2 = 1 if src_2 == "True" else 0
        
        if isinstance(src_1, str) or isinstance(src_2, str):
            self.Registers.set_register(destination_register, "%s & %s" % (src_1, src_2))
        else:
            self.Registers.set_register(destination_register, src_1 & src_2)

    def updateIndent(self):
        self.indents = ''.join(['\t'] * self.indent_level)

    def parse_tbz(self):
        ### FIX PARSING OF TBZ!!!!
        #### LETS GOOOOOOOOOOOOOO!!!!!!!
        instr = self.instructions[self.current_instruction]
        entries = [y.strip()[:-1] if y.endswith(",") else y.strip() for y in instr["opcode"].split(' ')[1:]]

        val = self.Registers.get_register(entries[0])
        tbz_offset_index = next((idx for (idx, d) in enumerate(self.instructions) if d["offset"] == instr["jump"]))
        
        self.output.append("%sif %s {" % (self.indents, val))
        after = StatusFunction(self.r2, copy.copy(self.Registers), self.instructions, self.fighter_register, self.current_instruction + 1, self.tmp, self.indent_level + 1, self.depth + 1, self.offsets_at_b_or_tbz, tbz_offset_index)
        after.run()
        

        next_stop = None

        for z in range(after.current_instruction, len(self.instructions)):
            if self.instructions[z]["offset"] in self.offsets_at_b_or_tbz:
                next_stop = z
                break

        self.current_instruction = after.current_instruction
        self.output = self.output + after.output
        

        if self.current_instruction != tbz_offset_index:
            branch = StatusFunction(self.r2, copy.copy(self.Registers), self.instructions, self.fighter_register, tbz_offset_index, self.tmp, self.indent_level + 1, self.depth + 1, self.offsets_at_b_or_tbz, next_stop)
            branch.run()
            
            if len(branch.output) > 0:
                self.output.append("%s} else {" % self.indents)
                self.output = self.output + branch.output
        self.output.append("%s}" % self.indents)

        # print('\n'.join(self.output))
        # print(self.instructions[self.current_instruction])


    def parse_movk(self):
        instr = self.instructions[self.current_instruction]["opcode"]
        
        entries = [y.strip()[:-1] if y.endswith(",") else y.strip() for y in instr.split(' ', 3)[1:]]
        
        src = self.parse_entry(entries[0])

        val = self.parse_entry(entries[1])

        if entries[2] == "lsl 16":
            src += (val << 16)
        elif entries[2] == "lsl 32":
            src += (val << 32)        

        self.Registers.set_register(entries[0], src)

    def parse_fmov(self):
        instr = self.instructions[self.current_instruction]["opcode"]
        entries = [y.strip()[:-1] if y.endswith(",") else y.strip() for y in instr.split(' ', 3)[1:]]
        dest_reg = entries[0]
        val = self.parse_entry(entries[1])
        self.Registers.set_register(dest_reg, val)

    def args_to_string(self) -> str:
        output = []
        for arg in self.args:
            if isinstance(arg, bool):
                output.append("%s" % (str(arg).lower()))
            elif isinstance(arg, list):
                tmp = ["*%s" % x for x in arg]
                output.append("(%s) as u64" % (' | '.join(tmp)))
            else:
                arg = str(arg)
                if Labels.is_lua_const(arg):
                    arg = "*%s" % arg
                if "SITUATION_KIND_" in arg:
                    arg = "SituationKind(%s)" % arg
                elif "GROUND_CLIFF_CHECK_" in arg:
                    arg = "GroundCliffCheckKind(%s)" % arg
                arg = "%s" % (arg)
                output.append(arg)
        return ', '.join(["%s.into()" % x for x in output])

    def fighter_args_to_string(self) -> str:
        output = []
        for arg in self.args:
            if isinstance(arg, bool):
                output.append("%s%s" % (self.indents, str(arg).lower()))
            elif isinstance(arg, list):
                tmp = ["%s" % x for x in arg]
                output.append("%s(%s) as u64" % (self.indents, ' | '.join(tmp)))
            else:
                arg = str(arg)
                arg = "%s%s" % (self.indents, arg)
                output.append(arg)
        return ',\n\t'.join(["(%s).into()" % x.strip() for x in output])

    @staticmethod
    def get_new_var_name() -> str:
        ret = "tmp_%s" % (StatusFunction.tmp)
        StatusFunction.tmp += 1
        return ret
    
    def get_var_name(self) -> str:
        return StatusFunction.get_new_var_name()

    def parse_ret(self):
        if len(self.args) == 1:
            self.output.append("%s%s.into()" % (self.indents, self.args[0]))

    def parse_bl(self):
        disasm = self.instructions[self.current_instruction]["disasm"]
        
        if "0x" in disasm:
            try:
                offset = disasm[disasm.index('0x'):].strip()
                import_name = Labels.imports_table[int(offset[2:], 16)]
                disasm = "%s %s" % (disasm[:disasm.index('0x')].strip(), import_name)
            except:
                pass

        if "lua2cpp::L2CFighterCommon::" in disasm:
            function_name = disasm[disasm.rindex('lua2cpp::L2CFighterCommon::') + len("lua2cpp::L2CFighterCommon::"):disasm.index('(')]
            self.output.append("%sfighter.%s();" % (self.indents, function_name))
        elif "lua2cpp::L2CFighterBase::" in disasm:
            function_name = disasm[disasm.rindex('lua2cpp::L2CFighterBase::') + len("lua2cpp::L2CFighterBase::"):disasm.index('(')]
            self.output.append("%sfighter.%s(%s);" % (self.indents, function_name, self.fighter_args_to_string()))
        elif "app::lua_bind::" in disasm:
            function_name = disasm[disasm.index('app::lua_bind::') + len('app::lua_bind::'):disasm.index('_impl')].replace("__", "::").replace("_impl", "")
            Labels.generate_function(self, function_name)
            self.args.clear()
        elif "app::sv_kinetic_energy::" in disasm:
            function_name = disasm[disasm.index('app::') + len('app::'):disasm.index('(lua_')]
            Labels.generate_function(self, function_name)
            self.args.clear()
        elif "L2CValue(int)" in disasm:
            self.args.append(self.Registers.get_register('w1'))
        elif "L2CValue(bool)" in disasm:
            self.args.append(bool(self.Registers.get_register('w1')))
        elif "L2CValue::operator bool() const" in disasm:
            var = self.get_var_name()
            self.output.append("%slet %s = %s;" % (self.indents, var, self.Registers.get_register('x1')))
            self.Registers.set_register('x0', var)
        elif "operator+" in disasm:
            var = StatusFunction.get_new_var_name()
            line = "%slet %s = %s + %s;" % (self.indents, var, self.args[0], self.args[1])
            self.output.append(line)
            self.Registers.set_register('w0', var)
            self.args.clear()
        elif "operator-" in disasm:
            var = StatusFunction.get_new_var_name()
            # print(self.args)
            # print(self.Registers.registers)
            # print(disasm)
            line = "%slet %s = %s * -1;" % (self.indents, var, self.args[0])
            self.output.append(line)
            self.Registers.set_register('w0', var)
            self.args.clear()
        elif "operator*" in disasm:            
            # print(self.args)
            # print(self.Registers.registers)
            # print(self.instructions[self.current_instruction])
            # exit()
            var = StatusFunction.get_new_var_name()
            # line = "%slet %s = %s * %s;" % (self.indents, var, self.args[0], self.args[1])
            # self.output.append(line)
            line = "%slet %s = todo!();" % (self.indents, var)
            self.output.append(line)
            self.Registers.set_register('w0', var)
            self.args.clear()
        elif "operator=" in disasm:
            # print(self.args)
            # print(self.Registers.registers)
            # exit()
            var = StatusFunction.get_new_var_name()
            line = "%slet %s = %s == %s;" % (self.indents, var, self.args[0], self.Registers.get_register('x0'))
            self.output.append(line)
            self.Registers.set_register('w0', var)
            self.args.clear()
        elif "operator/" in disasm:
            var = StatusFunction.get_new_var_name()
            line = "%slet %s = %s / %s;" % (self.indents, var, self.args[0], self.args[1])
            self.output.append(line)
            self.Registers.set_register('w0', var)
            self.args.clear()
        elif "operator<=" in disasm:
            var = StatusFunction.get_new_var_name()
            # line = "%slet %s = %s <= %s;" % (self.indents, var, self.args[0], self.args[1])
            # self.output.append(line)
            line = "%slet %s = todo!();" % (self.indents, var)
            self.output.append(line)
            self.Registers.set_register('w0', var)
            self.args.clear()
        elif "operator<" in disasm:
            var = StatusFunction.get_new_var_name()
            # line = "%slet %s = %s < %s;" % (self.indents, var, self.args[0], self.args[1])
            # self.output.append(line)
            line = "%slet %s = todo!();" % (self.indents, var)
            self.output.append(line)
            self.Registers.set_register('w0', var)
            self.args.clear()
        elif "operator!" in disasm:
            var = StatusFunction.get_new_var_name()
            line = "%slet %s = %s != true;" % (self.indents, var, self.args[0], self.args[1])
            self.output.append(line)
            self.Registers.set_register('w0', var)
            self.args.clear()
        elif "L2CValue::L2CValue(float)" in disasm:
            self.args.append(self.Registers.get_register('s0'))
        elif "L2CValue::L2CValue(phx::Hash40)" in disasm:
            self.args.append(hex(self.Registers.get_register('x1')))
        elif "L2CValue::operator[]" in disasm:
            src = self.Registers.get_register('x0')
            dest = self.Registers.get_register('x1')
            self.Registers.set_register('x0', "%s[%s]" % (src, hex(dest)))
        elif "L2CValue::operator==" in disasm:
            var = StatusFunction.get_new_var_name()
            line = "%slet %s = %s == *%s;" % (self.indents, var, self.Registers.get_register('x0'), self.args[0])
            self.output.append(line)
            self.Registers.set_register('w0', var)
            self.args.clear()
        elif "L2CAgent::clear_lua_stack" in disasm:
            self.output.append("%sfighter.clear_lua_stack()" % self.indents)
        elif "L2CAgent::push_lua_stack" in disasm:
            if len(self.args) >= 1: # Get rid of this later once we properly implement all lua related functions
                self.output.append("%sfighter.push_lua_stack(&mut %s.into())" % (self.indents, self.args[0]))
                self.args = self.args[1:]
        # elif "L2CValue::as_integer" in disasm:
        #     var = self.get_var_name()
        #     self.output.append("%slet %s = %s;" % (self.indents, var, self.args[0]))
        #     self.Registers.set_register('x0', var)
        elif "~L2CValue" in disasm: # ~L2CValue --- Fix hardcoded issue
            self.args.clear()
        else:
            disasm = self.instructions[self.current_instruction]["disasm"]
            # print(self.r2.cmdj('s {0};aF;pdfj'.format(hex(0x0))))
            fun_offset = disasm.split(' ')[1]
            if fun_offset.startswith("fcn.") or fun_offset.startswith('0x'):
                if fun_offset.startswith("fcn."):
                    fun_offset = int(fun_offset[4:], 16)
                else:
                    fun_offset = int(fun_offset[2:], 16)
                inner_fun = StatusFunction(self.r2, Registers(), self.r2.cmdj('s {0};aF;pdfj'.format(hex(fun_offset)))["ops"], self.fighter_register, 0, self.tmp, self.indent_level)
                inner_fun.run()
                self.output.extend(inner_fun.output)
        if self.current_instruction == (len(self.instructions) - 1):
            self.parse_ret()

    def run(self):
        self.current_instruction = self.start_instruction

        # if len(self.offsets_at_b_or_tbz) != 0:
        #     print('\n', [hex(x) for x in self.offsets_at_b_or_tbz])
        #     exit()

        while self.current_instruction < len(self.instructions):

            if self.stop_at_idx != None:
                if self.current_instruction == self.stop_at_idx:
                    break

            op_code = self.instructions[self.current_instruction]["opcode"].split(" ")[0]

            if op_code == "movz" or op_code == "mov":
                self.parse_movz()
            elif op_code == "movk":
                self.parse_movk()
            elif op_code == "fmov":
                self.parse_fmov()
            elif op_code == "adrp":
                self.parse_adrp()
            elif op_code == "ldr":
                self.parse_ldr()
            elif op_code == "add":
                self.parse_add()
            elif op_code == "orr":
                self.parse_orr()
            elif op_code == "and":
                self.parse_and()
            elif op_code == "tbz":
                self.parse_tbz()
                continue
            elif op_code == "bl":
                self.parse_bl()
            elif op_code == "b":
                if "sym" in self.instructions[self.current_instruction]["disasm"]:
                    self.parse_bl()
                    if self.current_instruction == len(self.instructions) - 1:
                        self.parse_ret()
                else:
                    if self.depth >= 1:
                        self.current_instruction = next((idx for (idx, d) in enumerate(self.instructions) if d["offset"] == self.instructions[self.current_instruction]["jump"]))
                        break
                    else:
                        self.current_instruction = next((idx for (idx, d) in enumerate(self.instructions) if d["offset"] == self.instructions[self.current_instruction]["jump"]))
                        continue
            elif op_code == "ret":
                self.parse_ret()

            self.current_instruction += 1




# SetStatusFunction Class
# Description: A class to scan the function that sets up the Status and their Pre/Main/Loop/End
class SetStatusFunction:
    def __init__(self, r2, regs, instructions) -> None:
        self.output = []
        self.Registers = regs
        self.current_instruction = 0
        self.instructions = instructions
        self.stack = {}
        self.sp = 0
        self.r2 = r2
        self.args = []

    def parse_movz(self):
        instr = self.instructions[self.current_instruction]
        register = instr["opcode"].split(' ')[1].strip()[:-1]
        value = instr["opcode"].split(' ')[2].strip()

        if "sp" in value:
            return

        if value.startswith("0x"):
            value = int(value[2:], 16)
        elif value.startswith("x") or value.startswith("w") or value.startswith("s"):
            value = self.Registers.get_register(value)
        else:
            value = int(value)

        if register[0] == "w":
            register = "x" + register[1:]

        self.Registers.set_register(register, value)

    def parse_adrp(self):
        instr = self.instructions[self.current_instruction]
        register = instr["opcode"].split(' ')[1].strip()[:-1]
        value = int(instr["opcode"].split(' ')[2].strip()[2:], 16)
        self.Registers.set_register(register, value)

    def parse_ldr(self):
        instr = self.instructions[self.current_instruction]
        
        if "sp" in instr["opcode"]:
            return
        
        dest_register = instr["opcode"].split(' ')[1].strip()[:-1]

        (src_register, offset) = instr["opcode"].split(' ', 2)[-1].split(',')
        src_register = src_register[1:].strip()

        offset = offset.strip()[:-1]

        if offset.startswith("0x"):
            offset = int(offset.strip()[2:], 16)
        elif offset.startswith("x"):
            offset = self.Registers.get_register(offset)


        type = dest_register[0]
        
        if type == "x":
            data = self.r2.cmd('s {0};pf {1}'.format(self.Registers.get_register(src_register) + offset, 'i'))
            data = int(data[data.index('=') + 1:].strip())
            self.Registers.set_register(dest_register, data)
        elif type == "w":
            idx = int((offset / 4)) + 1
            if idx in Labels.constants:
                self.Registers.set_register(dest_register, Labels.constants[idx])
            else:
                self.Registers.set_register(dest_register, 0)

        # self.Registers.set_register(register, value)

    def parse_add(self):
        instr = self.instructions[self.current_instruction]["opcode"]
        dest_register = instr.split(' ')[1][:-1]
        src_1 = instr.split(' ')[2][:-1]
        src_2 = instr.split(' ')[3]
        
        if "sp" in src_1 or "sp" in src_2:
            return

        if "sp" in src_1:
            src_1 = self.sp
        elif src_1.startswith("0x"):
            src_1 = int(src_1[2:], 16)
        else:
            src_1 = self.Registers.get_register(src_1)

        if "sp" in src_2:
            src_2 = self.sp
        elif src_2.startswith("0x"):
            src_2 = int(src_2[2:], 16)
        else:
            src_2 = self.Registers.get_register(src_2)
        
        if dest_register == "sp":
            self.stack[self.sp] = src_1 + src_2
        else:
            self.Registers.set_register(dest_register, src_1 + src_2)
        
    # Returns the following:
    # [FIGHTER_STATUS, LUA_STATUS_SCRIPT_FUNC_EXEC_XXXXX, FUNCTION_OFFSET]
    def run(self):
        for x in range(len(self.instructions)):
            self.current_instruction = x
            op_code = self.instructions[x]["opcode"].split(" ")[0]

            if op_code == "movz" or op_code == "mov":
                self.parse_movz()
            elif op_code == "adrp":
                self.parse_adrp()
            elif op_code == "ldr":
                self.parse_ldr()
            elif op_code == "add":
                self.parse_add()
            elif op_code == "bl":
                if "sv_set_status_func" in self.instructions[x]["disasm"]:
                    self.args.append("710%s" % hex(self.Registers.get_register('x3'))[2:].rjust(7, '0'))
                    self.output.append(self.args.copy())
                    self.args.clear()
                elif "L2CValue(int)" in self.instructions[x]["disasm"]:
                    self.args.append(self.Registers.get_register('w1'))
                elif "0x12bcc0" in self.instructions[x]["disasm"]: # ~L2CValue
                    self.args.clear()
        with open('output.json', "w+") as f:
            json.dump(self.output, f, indent=4)
import zlib, json

class Labels:
    return_types = {
        "x": ["u64", "i64"],
        "w": ["bool", "i32", "u32", "libc::c_uchar"],
        "s": ["f8", "f16", "f32", "f64"]
    }
    labels = {}
    constants = {}
    l2c_fighter_common_functions = {}
    lua_bind_functions = {}
    fighter_register_offsets = {
         0x8: "lua_state_agent",
        0x38: "battle_object",
        0x40: "module_accessor",
        0xC8: "global_table",
    }
    imports_table = {}
    is_loaded = False

    @staticmethod
    def make_hash40(source_str: str):
        return (len(source_str) << 32) + zlib.crc32(source_str.encode())

    @staticmethod
    def setup():
        if Labels.is_loaded:
            return 
        Labels.labels = {}
        for x in open("article_labels.txt").readlines():
            x = x.strip().split(',', 2)
            hash = int(x[0][2:], 16)
            Labels.labels[hash] = x[1]
            split = x[1].split('_', 2)
            for z in split:
                hash = Labels.make_hash40(z)
                Labels.labels[hash] = z
        for x in open("ParamLabels.csv").readlines():
            x = x.strip().split(',', 2)
            hash = int(x[0][2:], 16)
            Labels.labels[hash] = x[1]
        idx = 1
        for x in open("const_value_table_13.0.1.csv").readlines():
            x = x.strip().split(',', 2)
            # hash = int(x[0][2:], 16)
            Labels.constants[idx] = x[1]
            idx += 1
        for x in open("l2c_fighter_funcs.csv").readlines():
            x = x.strip()
            function_name = x.split(',')[0].strip()
            args = [y.strip() for y in x.split(',')[1:]]
            Labels.l2c_fighter_common_functions[function_name] = args
        with open("lua_bind_funcs.json", "r") as f:
            Labels.lua_bind_functions = json.load(f)
        Labels.is_loaded = True

    @staticmethod
    def get_label(hash) -> str:
        return Labels.labels[hash]

    @staticmethod
    def is_lua_const(const) -> str:
        return True if next((entry for entry in Labels.constants if Labels.constants[entry] == const), -1) != -1 else False

    @staticmethod
    def get_fighter_offset(offset) -> str:
        if isinstance(offset, str):
            if offset.startswith("0x"):
                offset = int(offset[2:], 16)
            else:
                offset = int(offset)
        return "fighter.%s" % Labels.fighter_register_offsets[offset]
    
    @staticmethod
    def args_to_string(function_def_args, lua_args, registers, indents):
        output = []

        for idx in range(len(function_def_args)):
            arg = ""
            if idx < len(lua_args):
                arg = lua_args[idx]
            else:
                info = function_def_args[idx]
                reg = info
                type = ""
                if "-" in info:
                    spl = info.split('-', 2)
                    reg = spl[0]
                    type = spl[1]
                arg = registers.get_register(reg)
                if type == "bool":
                    arg = bool(arg)

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
        # output[0] = "\t%s" % output[0]
        return ', '.join(output)

        # print(function_def_args)
        # print(lua_args)
        # print(registers.registers)

    @staticmethod
    def setup_imports(data):
        Labels.imports_table = {}
        for x in data:
            try:
                Labels.imports_table[x["plt"]] = x["name"]
            except:
                pass
    # Description: 
    # Returns: 
    @staticmethod
    def generate_function(st, function_name):
        # rets = []
        if function_name in Labels.lua_bind_functions:
            # Dump all type of returns 
            # for x in Labels.lua_bind_functions:
            #     if "ret" in Labels.lua_bind_functions[x]:
            #         if Labels.lua_bind_functions[x]["ret"] not in rets:
            #             rets.append(Labels.lua_bind_functions[x]["ret"])
            # print('\n'.join(rets))
            # print()
            # print(hex(st.instructions[st.current_instruction]["offset"]))
            # print(function_name)
            function_def = Labels.lua_bind_functions[function_name]
            # print(function_def)
            # print(st.args)
            # print(st.Registers.registers)
            # print()

            args = Labels.args_to_string(function_def["args"], st.args, st.Registers, st.indents)
            # print(args)


            if function_name == "MotionModule::change_motion":
                pass
                # exit()

            var = ""
            if "ret" in function_def:
                if st.return_register_is_used(0, st.current_instruction + 1, 10):
                    var = st.get_var_name()
                    if function_def["ret"] in Labels.return_types["w"]:
                        st.Registers.set_register('w0', var)
                    elif function_def["ret"] in Labels.return_types["s"]:
                        st.Registers.set_register('s0', var)
                    else: # Assume its 64-bit if its not 32 or float
                        st.Registers.set_register('x0', var)
            
            if var != "":
                st.output.append("%slet %s = %s(%s);" % (st.indents, var, function_name, args))
            else:
                st.output.append("%s%s(%s);" % (st.indents, function_name, args))
        else:
            output_args = st.args_to_string()
            if "is_" in function_name:
                var = st.get_var_name()
                st.Registers.set_register('w0', var)
                st.output.append("%slet %s = %s(%s);" % (st.indents, var, function_name, output_args))
            else:
                st.output.append("%s%s(%s);" % (st.indents, function_name, output_args))

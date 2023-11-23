import json

# out = ""
# with open("l2cfighter_funcs.rs", "r") as f:
#     for x in f.readlines():
#         x = x.strip()
#         if "pub unsafe fn" in x:
#             function_name = x[len("pub unsafe fn "):x.index('(')]
#             args = [y[y.rindex('::') + 2:].strip() if "::" in y else y.strip() for y in x[x.index('(') + 1:x.index(')')].replace("&mut ", "").split(',')]
#             out += "%s,%s\n" % (function_name, ','.join(args))

# open("l2c_fighter_funcs.csv", "w+").write(out)
# 

all_args = []

data = {}
with open("motion_module.rs", "r") as f:
    lines = f.readlines()
    x = 0
    prefix = "MotionModule"
    while x < len(lines):
        line = lines[x].strip()
        
        if "pub mod " in line:
            prefix = line[len("pub mod "):line.index('{') - 1]
            print(prefix)
        elif "///" in line:
            pass
        else:
            if "pub fn" in line:
                out = {}
                function_name = "%s::%s" % (prefix, line[len("pub fn "):line.index('(')])
                args = []
                if ')' in line:
                    args.append(line[line.index('(') + 1:line.index(')')])
                else:
                    while ')' not in line:
                        line = lines[x + 1]
                        args.append(line.strip())
                        x += 1
                    args = args[:-1]
                args = [y.strip()[:-1] if y.endswith(",") else y.strip() for y in args]
                print(function_name)
                x_count = 0
                w_count = 0
                s_count = 0
                for y in range(len(args)):
                    print(args)
                    if args[y][args[y].index(':') + 2:] not in all_args:
                        all_args.append(args[y][args[y].index(':') + 2:])
                    if "*" in args[y] or ": libc::c_long" in args[y] or ": libc::c_ulong" in args[y]:
                        args[y] = "x%s" % x_count
                        x_count += 1
                    elif ": bool" in args[y]:
                        args[y] = "w%s-bool" % w_count
                        w_count += 1
                    elif ": libc::c_int" in args[y] or ": libc::c_uint" in args[y]:
                        args[y] = "w%s" % w_count
                        w_count += 1
                    elif ": f32" in args[y]:
                        args[y] = "s%s" % s_count
                        s_count += 1
                    else:
                        args[y] = "x%s-%s" % (x_count, args[y][args[y].index(':') + 2:])
                        x_count += 1
                out["args"] = args
                if "->" in line:
                    out["ret"] = line[line.index('->') + 2:line.index(';')].strip()
                data[function_name] = out
        x += 1
# open("l2c_fighter_funcs.csv", "w+").write(out)
print('\n'.join(all_args))

with open("motion_module_data.json", "w+") as f:
    json.dump(data, f, indent=4)
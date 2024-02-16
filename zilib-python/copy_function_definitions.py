#!/usr/bin/env python3

# Is there already an established way to do this? I would want to know, but too
# lazy to look it up.

import glob
import os
import sys
import re

def join_space(s: str) -> str:
    return re.sub(r"\s+", " ", s, re.DOTALL).strip()

ignored_functions = set([
    "binary_search_file",
    "jyutping_validator",
    "radical_char_cmp",
    "radical_cmp",
    "radical_label_to_chars",
    ])
ignored_rust_files = set([
    "data.rs",
    ])
# Copy all function definitions from source path to destination file
def main():
    if len(sys.argv) < 2:
        print("Usage: copy_function_definitions.py <source_dir> [<destination_file>]")
        sys.exit(1)

    source_directory = sys.argv[1]
    destination_file = sys.argv[2] if len(sys.argv) == 3 else "src/lib.rs"
    print("Writing to " + destination_file)

    destination_lines = open(destination_file, "r").readlines()

    function_wrappers_start_idx = destination_lines.index("/* START_OF_GENERATED_FUNCTION_WRAPPERS */\n")
    function_wrappers_end_idx = destination_lines.index("/* END_OF_GENERATED_FUNCTION_WRAPPERS */\n")
    function_add_start_idx = destination_lines.index("    /* START_OF_GENERATED_ADD_FUNCTIONS */\n")
    function_add_end_idx = destination_lines.index("    /* END_OF_GENERATED_ADD_FUNCTIONS */\n")

    funcs = []

    with open(destination_file, "w") as dest:
        for line in destination_lines[:function_wrappers_start_idx+1]:
            dest.write(line)

        for file in glob.glob(source_directory + "/src/*.rs"):
            if os.path.basename(file) in ignored_rust_files:
                continue
            # Remember doc comments lines here
            comments = []
            base_name = os.path.basename(file).split(".")[0]
            with open(file, "r") as f:
                while line := f.readline():
                    if line.startswith("///"):
                        comments.append(line)
                    elif line.strip() == "":
                        pass
                    elif line.startswith("pub fn"):
                        for comment in comments:
                            dest.write(comment)
                        comments = []

                        signature = line
                        if '{' not in signature:
                            while line := f.readline():
                                signature += line
                                if '{' in line:
                                    break
                        func_name_args = signature.split("->")[0]

                        name, argstr = re.match(r"pub +fn +(\w+)\((.*)\)", func_name_args, re.DOTALL).groups()
                        if name in ignored_functions:
                            continue

                        no_type_args = []
                        # s:&str, => s
                        for arg in argstr.split(","):
                            var_name = re.sub(r":.*", "", arg).strip()
                            if '&Hash' in arg:
                                no_type_args.append(var_name + ".as_ref()")
                            else:
                                no_type_args.append(var_name)

                        dest.write("#[pyfunction]\n")
                        dest.write(join_space(signature.replace("&Hash", "Hash")) + "\n")
                        dest.write(f"    {base_name}::{name}({', '.join(no_type_args)})\n")
                        dest.write("}\n")

                        funcs.append(name)

                    else:
                        comments = []

        for line in destination_lines[function_wrappers_end_idx:function_add_start_idx+1]:
            dest.write(line)

        for func in funcs:
            dest.write(f"    m.add_function(wrap_pyfunction!({func}, m)?)?;\n")

        for line in destination_lines[function_add_end_idx:]:
            dest.write(line)


if __name__ == "__main__":
    main()

# dacho/compile_shaders.py

import os
from shutil import which


SHADER_COMPILER = "glslc"
SHADER_ROOT     = "assets/shaders"


class Color:
    red   = "\033[31;1m"
    green = "\033[32;1m"
    reset = "\033[0m"


def main():
    for shader in os.listdir(SHADER_ROOT):
        shader_path = f"{SHADER_ROOT}/{shader}"

        if not os.path.isdir(shader_path):
            continue

        bin_directory = f"{shader_path}/bin"

        if not os.path.exists(bin_directory):
            os.mkdir(bin_directory)

        for module in os.listdir(shader_path):
            module_path = f"{shader_path}/{module}"

            if not os.path.isfile(module_path):
                continue

            spir_v = f"{bin_directory}/{module.split('.')[1]}.spv"
            status = "recompiled" if os.path.exists(spir_v) else "compiled"

            if not os.system(f"{SHADER_COMPILER} {module_path} -o {spir_v}"):
                print(f"{Color.green}{status}{Color.reset} {module}")
            else:
                print(f"{Color.red}failed{Color.reset} to compile {module}")


if __name__ == "__main__":
    if which(SHADER_COMPILER) is None:
        exit(f"{Color.red}{SHADER_COMPILER}{Color.reset} is required to compile shaders")

    main()


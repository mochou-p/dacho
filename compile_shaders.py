# dacho/compile_shaders.py

from os      import listdir, mkdir, system
from os.path import exists, isdir, isfile
from shutil  import which


SHADER_COMPILER = "glslc"
SHADER_ROOT     = "assets/shaders"


class Color:
    red   = "\033[31;1m"
    green = "\033[32;1m"
    reset = "\033[0m"


def main():
    for shader in listdir(SHADER_ROOT):
        shader_path = f"{SHADER_ROOT}/{shader}"

        if not isdir(shader_path):
            continue

        bin_directory = f"{shader_path}/bin"

        if not exists(bin_directory):
            mkdir(bin_directory)

        for module in listdir(shader_path):
            module_path = f"{shader_path}/{module}"

            if not isfile(module_path):
                continue

            spir_v = f"{bin_directory}/{module.split('.')[1]}.spv"
            status = "recompiled" if exists(spir_v) else "compiled"

            if system(f"{SHADER_COMPILER} {module_path} -o {spir_v}"):
                print(f"{Color.red}failed{Color.reset} to compile {module}")
            else:
                print(f"{Color.green}{status}{Color.reset} {module}")


if __name__ == "__main__":
    if which(SHADER_COMPILER) is None:
        exit(f"{Color.red}{SHADER_COMPILER}{Color.reset} is required to compile shaders")

    main()


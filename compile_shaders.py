# dacho/compile_shaders.py

from concurrent.futures import ThreadPoolExecutor
from os                 import listdir, mkdir, system
from os.path            import exists, isdir, isfile
from shutil             import which


SHADER_COMPILER =  "glslc"
SHADER_ROOT     =  "assets/shaders"
SHADER_BIN_DIR  =  "bin"
SHADER_CACHE    = f"{SHADER_ROOT}/{SHADER_BIN_DIR}"
WS              =  " " * 10

class Color:
    red   = "\033[31;1m"
    cyan  = "\033[36;1m"
    reset = "\033[0m"


def compile_shader(shader):
    shader_path = f"{SHADER_ROOT}/{shader}"

    if not isdir(shader_path):
        return

    for module in listdir(shader_path):
        module_path = f"{shader_path}/{module}"

        if not isfile(module_path):
            continue

        spir_v = f"{SHADER_CACHE}/{module}.spv"
        status = "Recompiled" if exists(spir_v) else "Compiled"

        if system(f"{SHADER_COMPILER} {module_path} -o {spir_v}"):
            print(f"{WS}{Color.red}Error{Color.reset} Failed to compile `{module}`")
        else:
            print(f"{WS}{Color.cyan}Info{Color.reset} {status} `{module}`")


def main():
    if not exists(SHADER_CACHE):
        mkdir(SHADER_CACHE)

    shaders = [shader for shader in listdir(SHADER_ROOT) if shader != SHADER_BIN_DIR]

    with ThreadPoolExecutor() as tpe:
        tpe.map(compile_shader, shaders)


if __name__ == "__main__":
    if which(SHADER_COMPILER) is None:
        exit(f"{WS}{Color.red}Error{Color.reset} Failed to run `{__file__.split('/')[-1]}` ({SHADER_COMPILER} is missing)")

    try:
        main()
    except Exception as exception:
        print(f"{WS}{Color.red}Error{Color.reset} Failed to complete `{__file__.split('/')[-1]}` ({exception})")


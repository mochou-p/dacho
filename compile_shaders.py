# dacho/compile_shaders.py

from concurrent.futures import ThreadPoolExecutor
from os                 import listdir, mkdir, system
from os.path            import exists, isdir, isfile
from shutil             import which
from sys                import exit as sys_exit


COMPILER =  "glslc"
ROOT     =  "assets/shaders"
BIN_DIR  =  "bin"
CACHE    = f"{ROOT}/{BIN_DIR}"
WS       =  " " * 10

class Color:
    red   = "\033[31;1m"
    cyan  = "\033[36;1m"
    reset = "\033[0m"


def compile_shader(shader):
    shader_path = f"{ROOT}/{shader}"

    if not isdir(shader_path):
        return

    errors = 0

    for module in listdir(shader_path):
        module_path = f"{shader_path}/{module}"

        if not isfile(module_path):
            continue

        spir_v = f"{CACHE}/{module}.spv"
        status = "Recompiled" if exists(spir_v) else "Compiled"

        if system(f"{COMPILER} {module_path} -o {spir_v}"):
            errors += 1
        else:
            print(f"{WS}{Color.cyan}Info{Color.reset} {status} `{module}`")

    return errors


def main():
    if not exists(CACHE):
        mkdir(CACHE)

    shaders = [shader for shader in listdir(ROOT) if shader != BIN_DIR]

    with ThreadPoolExecutor() as tpe:
        errors = tpe.map(compile_shader, shaders)

    return sum(errors)


if __name__ == "__main__":
    if which(COMPILER) is None:
        sys_exit(f"{WS}{Color.red}Error{Color.reset} Failed to run `{__file__.split('/')[-1]}` ({COMPILER} is missing)")

    try:
        sys_exit(main())
    except Exception as exception:
        sys_exit(f"{WS}{Color.red}Error{Color.reset} Failed to complete `{__file__.split('/')[-1]}` ({exception})")


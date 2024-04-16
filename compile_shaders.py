# dacho/compile_shaders.py

from concurrent.futures import ThreadPoolExecutor
from os                 import listdir, mkdir, system
from os.path            import exists, isdir, isfile
from shutil             import which


SHADER_COMPILER =  "glslc"
SHADER_ROOT     =  "assets/shaders"
SHADER_BIN_DIR  =  "bin"
SHADER_CACHE    = f"{SHADER_ROOT}/{SHADER_BIN_DIR}"


class Color:
    red   = "\033[31;1m"
    green = "\033[32;1m"
    reset = "\033[0m"


def compile_shader(shader):
    shader_path = f"{SHADER_ROOT}/{shader}"

    if not isdir(shader_path):
        return

    results = ""

    for module in listdir(shader_path):
        module_path = f"{shader_path}/{module}"

        if not isfile(module_path):
            continue

        spir_v = f"{SHADER_CACHE}/{module}.spv"
        status = "Recompiled" if exists(spir_v) else "Compiled"

        if system(f"{SHADER_COMPILER} {module_path} -o {spir_v}"):
            results += f"      {Color.red}Failed{Color.reset} to compile `{module}`\n"
        else:
            results += f"      {Color.green}{status}{Color.reset} `{module}`\n"

    return results[:-1]


def main():
    if not exists(SHADER_CACHE):
        mkdir(SHADER_CACHE)

    shaders = []

    for shader in listdir(SHADER_ROOT):
        if shader == SHADER_BIN_DIR:
            continue

        shaders.append(shader)

    with ThreadPoolExecutor() as tpe:
        outputs = tpe.map(compile_shader, shaders)

    [print(output) for output in outputs]


if __name__ == "__main__":
    if which(SHADER_COMPILER) is None:
        exit(f"{Color.red}{SHADER_COMPILER}{Color.reset} is required to compile shaders")

    try:
        main()
    except Exception as exception:
        print(f"shader compilation script {Color.red}crashed{Color.reset}\n  {exception=}")


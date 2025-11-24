#!/usr/bin/env bash


COMPILER="glslang"

if ! command -v "${COMPILER}" > /dev/null; then
    echo -e "\x1b[31merror:\x1b[0m missing shader compiler \`${COMPILER}\`"
    exit 1;
fi

FLAGS="--quiet -V --client vulkan100 --target-env vulkan1.3 --glsl-version 460"

compile_shader_stage() {
    local name="$1"
    local stage="$2"

    echo -e "\x1b[92;1m   Compiling\x1b[0m \`${name}/${stage}.glsl\`"

    # TODO: build into target/ and remove *.spv from .gitignore
    eval "${COMPILER} ${FLAGS} -S ${stage} -o ${stage}.glsl.spv ${stage}.glsl"
}

compile_shader() {
    local name="$1"
  
    cd "${name}"

    compile_shader_stage "${name}" "vert"
    compile_shader_stage "${name}" "frag"
}

src="${BASH_SOURCE[0]}"
dir=$(dirname "${src}") # this is where the current script is located

cd "${dir}/assets/shaders/"

# compile in parallel (background subshells)
( compile_shader "test" ) &
wait


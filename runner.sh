#!/bin/bash
set -e

# actual call to pros cli
runner=("${PROS_CLI:=prosv5}" "upload" "--compress-bin")

# take the first argument
bin="${1}.bin"
# copy ELF to a binary and remove the hot reloading init
arm-none-eabi-objcopy -O binary -R .hot_init $1 $bin

# remove first arg
shift

cat << EOF > project.pros
{
    "py/object": "pros.conductor.project.Project",
    "py/state": {
        "project_name": "${1#*}",
        "target": "v5",
        "templates": {
            "kernel": {
                "location": "",
                "metadata": {
                    "origin": "pros-mainline",
                    "output": "${bin}"
                },
                "name": "kernel",
                "py/object": "pros.conductor.templates.local_template.LocalTemplate",
                "supported_kernels": null,
                "system_files": [],
                "target": "v5",
                "user_files": [],
                "version": "3.8.0"
            }
        },
        "upload_options": {}
    }
}
EOF

# if no arguments are provided then just do an upload
if [[ $# -eq 0 ]]; then
    eval $runner
    exit $?
fi

# stolen from https://stackoverflow.com/a/52538533
function token_quote {
    local quoted=()
    for token; do
        quoted+=( "$(printf '%q' "$token")" )
    done
    printf '"%s"' "${quoted[*]}"
}

# function to display usage
display_usage() {
    echo "Usage: cargo run ... -- [options]"
    echo ""
    echo "    Upload a binary to the V5 Brain"
    echo ""
    echo "Options:"
    echo "    --help                      Display this message"
    echo "    --port  <port_number>       Set the program slot on the GUI"
    echo "    --name  <text>              Set the remote program name"
    echo "    --after <run|screen|none>   Action to perform after upload"
    echo "    --serial                    Open serial monitor after upload"
    exit 1
}

declare -A flags
flags["slot"]=""
flags["name"]=""
flags["after"]=""
open_ser=false

# Argument parsing
while [[ $# -gt 0 ]]; do
    key="$1"

    case $key in
        --slot)
            flags["slot"]="$2"
            shift # past argument
            shift # past value
            ;;
        --name)
            flags["name"]="$(token_quote ${*:2})"
            shift # past argument
            shift # past value
            ;;
        --after)
            flags["after"]="$2"
            shift # past argument
            shift # past value
            ;;
        --serial)
            open_ser=true
            shift
            ;;
        --help)
            display_usage
            shift
            ;;
        *) # unknown option
            echo "Unknown flag: $key"
            echo
            display_usage
            ;;
    esac
done

arguments=()
for flag in "${!flags[@]}"; do
    if [ -n "${flags[$flag]}" ]; then
        arguments+=("--$flag" "${flags[$flag]}")
    fi
done

eval "${runner[@]}" "${arguments[@]}"

# open pros terminal if requested
if [ $open_ser = true ]; then
    echo "Opening pros serial terminal"
    eval ${PROS_CLI:=prosv5} terminal
fi

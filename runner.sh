#!/bin/bash
BIN="${1}.bin"
arm-none-eabi-objcopy -O binary -R .hot_init $1 $BIN

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
                    "output": "${BIN}"
                },
                "name": "kernel",
                "py/object": "pros.conductor.templates.local_template.LocalTemplate",
                "supported_kernels": null,
                "system_files": [],
                "target": "v5",
                "user_files": [],
                "version": "3.4.0"
            }
        },
        "upload_options": {}
    }
}
EOF

if [ -z $3 ]; then
	prosv5 upload
else
	prosv5 upload --slot $2 --name "${3}"
fi

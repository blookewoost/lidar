#!/bin/bash

dir="$1"

if [[ -d "$dir" ]]; then
    for file in "$dir"*; do
        if [[ -f "$file" && $file == *.laz ]]; then
            target=$(basename $file)
            newfile=${target//.laz/.las}
            echo -e "Converting: $file to ./data/$newfile"
            pdal translate $file ./data/$newfile
        fi
    done
else
    echo -e "$1 is not a directory!"
fi
#!/bin/bash
set -eu
set -o pipefail

workspace=$(realpath $(dirname $0)/../../)
basedir=$(dirname $(realpath $0))
scale_factors=(0.1 0.3 1 3 10 30 100 300 1000)

function merge_csv() {
    dir=$basedir/sf$1
    schema=$workspace/schemas/ldbc/ldbc_pathce_schema.json
    $workspace/tools/merge_csv.py -d $dir -s $schema -o $basedir/sf$sf.csv
}

for sf in ${scale_factors[@]}; do
    if ! [ -d $basedir/sf$sf ]; then
        continue
    fi
    echo "merge $sf"
    merge_csv $sf
done
#!/bin/bash
set -eu
set -o pipefail

basedir=$(dirname $(realpath $0))
confdir=$basedir/conf
scale_factors=(0.1 0.3 1 3 10 30 100 300 1000)

for sf in ${scale_factors[@]}; do
  cmp=$(echo "$sf > $MAX_SF" | bc)
  if [ $cmp == "1" ]; then
    continue
  fi
  outdir=$basedir/sf$sf
  rm -rf $outdir
  mkdir $outdir
  docker run \
    --rm \
    --mount type=bind,source=$outdir,target=/out \
    --mount type=bind,source=$confdir,target=/conf,readonly \
    -e SPARK_CONF_DIR=/conf \
    ldbc/datagen-standalone:0.5.1-19-829b7a04-2.12_spark3.2 \
    --parallelism $PARALLELISM \
    --memory 6g \
    -- \
    --format parquet \
    --scale-factor $sf \
    --epoch-millis \
    --explode-edges \
    --mode raw
  paths=$(find $outdir/graphs/parquet/raw/composite-projected-fk -name '*.parquet' -type f)
  for path in $paths; do
    label=$(basename $(dirname $path))
    newdir=$outdir/$label
    mkdir -p $newdir
    mv $path $newdir
  done
  rm -rf $outdir/graphs
done
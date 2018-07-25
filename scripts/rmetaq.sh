#!/bin/bash
# Query an audio file to get metadata
scr=$(readlink -f $0)
scrdir=`dirname $scr`
directory=$1
script=${2:-"$scrdir/metaq.sh"}
find "$directory" -type f -exec $script {} \;
#!/bin/bash
# Query an audio file to get metadata
directory=$1
find $directory -type f -exec ~/personal/ellington/scripts/metaq.sh {} \;
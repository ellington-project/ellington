#!/bin/bash
# Query an audio file to get metadata
audio=$1

if file --mime-type "$audio" | grep -q flac$; then
    echo "File is flac audio"
    metaflac --export-tags-to=- "$audio"
    echo ""
    exit 0
fi 

if file --mime-type "$audio" | grep -q mpeg$; then
    echo "File is mp3 audio"
    id3v2 --list "$audio"
    echo ""
    exit 0
fi 

if file --mime-type "$audio" | grep -q m4a$; then
    echo "File is alac audio"
    mp4file --list "$audio"
    mp4info "$audio"
    echo ""
    exit 0
fi 

echo "Not an audio file"

exit 1

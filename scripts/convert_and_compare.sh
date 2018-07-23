#!/bin/sh

f=$1

rm -f "$f.sraw" "$f.fraw"

time sox -V1 "$f" -r 44100 -e float -c 1 -b 32 -t raw "$f.sraw"

echo "Sox Result: $?"

time ffmpeg -i "$f" -f f32le -acodec pcm_f32le -ac 1 -ar 44100 "$f.fraw"

echo "ffmpeg Result: $?"

diff "$f.sraw" "$f.fraw"

ls -l "$f.sraw"
ls -l "$f.fraw"

xxd "$f.sraw" | head -n 10000 | tail -n 20
echo "---"
xxd "$f.fraw" | head -n 10000 | tail -n 20

sox -r 44100 -e float -c 1 -b 32 -t raw  "$f.fraw" "$f.mp3"


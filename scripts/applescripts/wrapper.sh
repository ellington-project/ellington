#!/bin/sh
# Wrapper.sh
# Queries properties for an audio file, using ellington.
# Updates the data passed into the script
# Ignores stderr

audiof=$1
updat=$2

echo "Querying: $audiof, updating $updat" >> /tmp/wrapperlog.txt
library="/Users/adam/Music/ellington/library.json"

export PATH=$PATH:/usr/local/bin/:/Users/adam/projects/bellson/bin/:/Users/adam/projects/ellington/target/release/

export RUST_LOG=ellington

if ! ellington query "$audiof" "$library" -m userdata -u "$updat" -o update -a 2>>/tmp/wrapperlog.txt ; then
	echo "$updat"
fi

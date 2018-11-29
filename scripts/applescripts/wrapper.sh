#!/bin/sh
# Wrapper.sh
# Queries properties for an audio file, using ellington.
# Updates the data passed into the script
# Ignores stderr

audiof=$1
updat=$2

echo "Querying: $audiof, updating $updat" >> /tmp/wrapperlog.txt
#  library="/Users/adam/Music/ellib.json"

case "$(uname -s)" in
    Darwin)
        echo "Mac OSX" >> /tmp/wrapperlog.txt
export PATH=$PATH:/usr/local/bin/:/Users/adam/projects/bellson/bin/:/Users/adam/projects/ellington/target/release/
        ;;

    Linux)
        echo "Linux" >> /tmp/wrapperlog.txt
export PATH=$PATH:/usr/local/bin/:/home/adam/personal/bellson/bin/:/home/adam/personal/ellington/target/release/
        ;;
    *)
        echo "Unknown OS" >> /tmp/wrapperlog.txt
        ;;
esac


ellington query "$audiof" "$library" -m userdata -u "$updat" -o update -a -f -p 2>>/tmp/wrapperlog.txt
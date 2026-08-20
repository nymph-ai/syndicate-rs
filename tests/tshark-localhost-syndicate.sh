#!/bin/sh
#
# Uses tshark (wireshark cli) to dump traffic on localhost at a particular port, and assumes
# the packets are preserves binary.

portnumber="${1:-9001}"
echo "Running tshark on port $portnumber" >&2
tshark -i lo \
       -l \
       -q \
       -f "tcp port $portnumber" \
       -T fields \
       -e ip.src -e tcp.srcport -e ip.dst -e tcp.dstport -e @tcp.payload | \
    while read -r src srcport dst dstport body; do
        if [ -n "$body" ]
        then
            echo
            echo "$(date --rfc-3339=ns)	$src:$srcport::$dst:$dstport"
	    echo $body
            echo $body | xxd -r -p | preserves-tool
        fi
    done
#!/usr/bin/env bash

set -euo pipefail

source "./scripts/tests/lib.sh"

# Download 10MB at a max rate of 1MB/s. Shouldn't take longer than 12 seconds (allows for 2s of restablishing)
docker compose exec -it client sh -c \
    "curl \
        --fail \
        --max-time 12 \
        --limit-rate 1M http://download.httpbin/bytes?num=10000000" > download.file &

DOWNLOAD_PID=$!

sleep 3 # Download a bit

docker network disconnect firezone_app firezone-client-1 # Disconnect the client
sleep 1

docker network connect firezone_app firezone-client-1 --ip 172.28.0.200 # Reconnect client with a different IP
sudo kill -s HUP "$(ps -C firezone-linux-client -o pid=)" # Send SIGHUP, triggering `reconnect` internally

wait $DOWNLOAD_PID || {
    echo "Download process failed"
    exit 1
}

known_checksum="f5e02aa71e67f41d79023a128ca35bad86cf7b6656967bfe0884b3a3c4325eaf"
computed_checksum=$(sha256sum download.file | awk '{ print $1 }')

if [[ "$computed_checksum" != "$known_checksum" ]]; then
    echo "Checksum of downloaded file does not match"
    exit 1
fi

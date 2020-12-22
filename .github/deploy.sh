#!/bin/bash

set -o errexit
set -o pipefail
set -o nounset

hook="$1"

http_code=$(curl --silent --output /dev/null --write-out '%{http_code}\n' -X POST "$hook")
if [[ $http_code -eq 204 ]]; then
    echo '👌'
else
    echo "🤬 $http_code"
fi

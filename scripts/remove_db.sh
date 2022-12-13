#! /usr/bin/env bash
set -x
set -eo pipefail

container_id=$(docker ps -aqf status=running)

>&2 echo "Stop docker ${container_id}"
docker stop ${container_id}

>&2 echo "Remove docker ${container_id}"
docker container rm ${container_id}

>&2 echo "Successfully removed the db"
#!/bin/bash
set -o errexit   # abort on nonzero exitstatus
set -o nounset   # abort on unbound variable
set -o pipefail  # don't hide errors within pipes

readonly DOCKER="docker"
readonly HIVE_IMAGE_TAG="apache/hive:4.0.0-beta-2-SNAPSHOT"
readonly CONTAINER_NAME="metastore-standalone"
readonly MODE="${1:-}"

container_exists() {
  local id
  id=$("${DOCKER}" container ls --all --quiet --filter "name=${CONTAINER_NAME}")
  if [ "$id" != "" ]
  then
    return 0
  fi

  return 1
}

usage() {
  echo ""
  echo "convenience script to run trino locally for testing"
  echo ""
  echo "usage: $0 start/stop/rm"
}

if [ -z "${MODE}" ]; then
  usage
  exit 0
fi

case "$MODE" in
"start")
  if container_exists
  then
    echo "Starting a previous container."
    ${DOCKER} start "${CONTAINER_NAME}"
  else
    ${DOCKER} run --name "${CONTAINER_NAME}" --detach --publish 9083:9083 \
      --env SERVICE_OPTS="-Dhive.root.logger=console" \
      --env SERVICE_NAME=metastore \
      ${HIVE_IMAGE_TAG} 
  fi
  ;;
"stop")
  ${DOCKER} kill "${CONTAINER_NAME}"
  ;;
"rm")
  ${DOCKER} rm "${CONTAINER_NAME}"
  ;;
*)
  usage
  ;;
esac

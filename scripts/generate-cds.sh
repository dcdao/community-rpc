#!/bin/sh
#

set -e

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORK_PATH=${BIN_PATH}/../

mkdir -p ${WORK_PATH}/dist
DEST_CONFIG=${WORK_PATH}/dist/cds.local.yaml

_gen_schema() {
  CHAIN=$1
  PROTOCOL=$2
  echo '{"chain": "'${CHAIN}'", "protocol": "'${PROTOCOL}'"}'
}

init() {
  > $DEST_CONFIG
  echo 'version_info: "0"' >> $DEST_CONFIG
  echo 'resources:' >> $DEST_CONFIG
}

parse() {
  CHAIN=$1
  echo "start parse ${CHAIN}"

  ## http
  HTTP_MD=${WORK_PATH}/provider/${CHAIN}/http.csv
  export SCHEMA_JSON=$(_gen_schema ${CHAIN} http)
  export ENDPOINTS=$(mlr --c2j cat ${HTTP_MD})

  cat ${WORK_PATH}/assets/cds.tpl.yaml \
    | gomplate \
    -d endpoints='env:/ENDPOINTS?type=application/json' \
    -d schema='env:/SCHEMA_JSON?type=application/json' \
    >> ${DEST_CONFIG}

  ## ---
  echo '' >> ${DEST_CONFIG}
  echo "parse ${CHAIN} http"

  ## websocket
  WS_MD=${WORK_PATH}/provider/${CHAIN}/websocket.csv
  export SCHEMA_JSON=$(_gen_schema ${CHAIN} ws)
  export ENDPOINTS=$(mlr --c2j cat ${WS_MD})

  cat ${WORK_PATH}/assets/cds.tpl.yaml \
    | gomplate \
    -d endpoints='env:/ENDPOINTS?type=application/json' \
    -d schema='env:/SCHEMA_JSON?type=application/json' \
    >> ${DEST_CONFIG}
  echo "parse ${CHAIN} ws"
}



main() {
  init
  parse crab
  parse darwinia
  echo "parse done, output to ${DEST_CONFIG}"
}

main

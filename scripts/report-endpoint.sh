#!/bin/bash
#

set -xe

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORK_PATH=${BIN_PATH}/../
DIST_PATH=${WORK_PATH}/dist


CHAIN=$1

if [ -z "${CHAIN}" ]; then
  echo 'Missing chain'
  exit 1
fi

_prepare() {
  mkdir -p ${DIST_PATH}
}

_generate_state() {
  ITEM_ENDPOINT=$1

  _jq() {
     echo ${ITEM_ENDPOINT} | base64 --decode | jq -r ${1}
  }

  STATE_CURL=${DIST_PATH}/state-http.txt
  STATE_CSV=${DIST_PATH}/state-http.csv
  > ${STATE_CURL}
  > ${STATE_CSV}

  curl -L -s \
    -X POST \
    -w '\n\n%{time_connect},%{time_starttransfer},%{time_total}\n' \
    -H 'Content-Type: application/json' \
    $(_jq '.endpoint') \
    -d '{"id":1,"jsonrpc":"2.0","method":"system_version"}' \
    > ${STATE_CURL}

  echo 'time_connect,time_starttransfer,time_total,available,node_version,alias,address,endpoint' > ${STATE_CSV}

  _NODE_VERSION="$(cat dist/state-http.txt | head -1 | jq -r '.result')"
  _AVAILABLE=$([ -n "$_NODE_VERSION" ] && echo true || echo false)
  _ALIAS=$(_jq '.alias' | xargs)
  _ADDRESS=$(_jq '.address' | xargs)
  _ENDPOINT=$(_jq '.endpoint' | xargs)
  STATE_VALUE="$(tail -1 ${STATE_CURL}),${_AVAILABLE},${_NODE_VERSION},${_ALIAS},${_ADDRESS},${_ENDPOINT}"
  echo "${STATE_VALUE}" >> ${STATE_CSV}
  STATE_JSON=$(mlr --c2j cat ${STATE_CSV})
  echo "$STATE_JSON"
}

generate_report() {
  PROVIDER_HTTP=${WORK_PATH}/provider/${CHAIN}/http.csv
  PROVIDER_WS=${WORK_PATH}/provider/${CHAIN}/websocket.csv

  REPORT_CSV=${DIST_PATH}/report.csv

  ENDPOINT_JSON_HTTP=$(mlr --c2j cat ${PROVIDER_HTTP})
  ENDPOINT_JSON_WS=$(mlr --c2j cat ${PROVIDER_WS})

  DAY=$(date +'%Y-%m-%d')
  HOURM=$(date +'%H%M')

  echo 'time_connect,time_starttransfer,time_total,available,node_version,alias,address,endpoint,protocol' > ${REPORT_CSV}
  for ITEM in $(echo $ENDPOINT_JSON_HTTP | jq -r '.[] | @base64'); do
    STATE_JSON=$(_generate_state $ITEM)
    CSV_LINE=$(echo $STATE_JSON | jq -r '.[] | join(",")')
    CSV_LINE="${CSV_LINE},http"
    echo ${CSV_LINE}
    echo "${CSV_LINE}" >> ${REPORT_CSV}
  done

  for ITEM in $(echo $ENDPOINT_JSON_WS | jq -r '.[] | @base64'); do
    STATE_JSON=$(_generate_state $ITEM)
    CSV_LINE=$(echo $STATE_JSON | jq -r '.[] | join(",")')
    CSV_LINE="${CSV_LINE},ws"
    echo ${CSV_LINE}
    echo "${CSV_LINE}" >> ${REPORT_CSV}
  done

  FINAL_REPORT_CSV=${WORK_PATH}/report/$DAY/${CHAIN}-${HOURM}.csv
  mkdir -p ${WORK_PATH}/report/$DAY
  mv ${REPORT_CSV} ${FINAL_REPORT_CSV}

  echo "Report: ${FINAL_REPORT_CSV}" > ${DIST_PATH}/latest.log
}

main() {
  _prepare
  generate_report
}

main

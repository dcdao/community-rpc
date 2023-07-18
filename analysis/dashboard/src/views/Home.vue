<template>
  <v-row>
    <v-col cols="12">
      <v-select
        label="Month"
        v-model="chosenMonth"
        :items="monthList"
        @update:model-value="changedMonth"
      />
    </v-col>
    <v-col cols="12">
      <v-table>
        <thead>
          <tr>
            <td>Cluster</td>
            <td>Endpoint</td>
            <td>Hostname</td>
            <td>Success(times)</td>
            <td>Error(times)</td>
            <td>Total(times)</td>
            <td>Timeout(times)</td>
            <td>Weight</td>
          </tr>
        </thead>
        <tbody>
        <tr v-for="item in clusterInfos" :key="item.id">
          <td>{{ item.name }}</td>
          <td>{{ item.endpoint }}</td>
          <td>{{ item.hostname }}</td>
          <td>{{ item.rq_success }}</td>
          <td>{{ item.rq_error }}</td>
          <td>{{ item.rq_total }}</td>
          <td>{{ item.rq_timeout }}</td>
          <td>{{ item.weight }}</td>
        </tr>
        </tbody>
      </v-table>
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>

import {onBeforeMount, ref} from "vue";
import moment from 'moment'
import axios from 'axios'

interface ClusterInfo {
  endpoint: string,
  hostname: string,
  id: string,
  name: string,
  rq_error: number,
  rq_success: number,
  rq_timeout: number,
  rq_total: number,
  weight: number,
}


const chosenMonth = ref('' as string);
const monthList = ref([] as string[]);
const clusterInfos = ref([] as ClusterInfo[]);
const base_url = {
  dev: 'http://60.214.102.126:10234',
  pro: 'https://community-rpc-api.darwiniacommunitydao.xyz',
  test: 'https://community-rpc-api.darwiniacommunitydao.xyz',
}

async function loadClusterInfo() {
  const PATH_URL = base_url[import.meta.env.VITE_API_BASEPATH ?? 'dev']
  try {
    const resp = await axios.get(`${PATH_URL}/cluster/${chosenMonth.value}/cluster.json?_=${+new Date()}`);
    clusterInfos.value = resp.data;
  } catch (e) {
    clusterInfos.value = [];
  }
}

async function changedMonth() {
  await loadClusterInfo();
}

async function initState() {
  const today = new Date()
  chosenMonth.value = moment(today).format('YYYY-MM');
  const months = [];
  for (let i = 0; i < 6; i++) {
    const d = new Date(today.getFullYear(), today.getMonth() - i, 1);
    // months.push(`${d.getFullYear()}-${d.getMonth()}`);
    months.push(moment(d).format('YYYY-MM'));
  }
  monthList.value = months;
  await loadClusterInfo();
  setInterval(() => {
    loadClusterInfo();
  }, 3000);
}

onBeforeMount(() => {
  initState();
});
</script>

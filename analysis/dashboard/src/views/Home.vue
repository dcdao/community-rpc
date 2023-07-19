<template>
  <v-row>
    <v-col cols="12">
      <h2>Cluster Info</h2>
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
          <th>Cluster</th>
          <th>Endpoint</th>
          <th>Hostname</th>
          <th>Success(times)</th>
          <th>Error(times)</th>
          <th>Total(times)</th>
          <th>Timeout(times)</th>
          <th>Weight</th>
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
    <v-divider/>
    <v-col cols="12" v-if="rewardReview.rewardTotal">
      <h2>Reward Preview</h2>
      <v-table>
        <thead>
        <tr>
          <th>Key</th>
          <th>Value</th>
        </tr>
        </thead>
        <tbody>
        <tr>
          <td>Reward Quarterly</td>
          <td>{{ rewardReview.rewardTotal }}</td>
        </tr>
        <tr>
          <td>Reward Month</td>
          <td>{{ rewardReview.rewardMonth }}</td>
        </tr>
        <tr>
          <td>Reward Reversed</td>
          <td>{{ rewardReview.rewardReversed }}</td>
        </tr>
        <tr>
          <td>Reward Available</td>
          <td>{{ rewardReview.rewardAvailable }}</td>
        </tr>
        </tbody>
      </v-table>
    </v-col>
    <v-col cols="12" v-for="(item, ix) in rewardReview.rewardClusters" :key="`reward-cluster-${ix}`">
      <v-card>
        <v-card-title>{{ item.name }}</v-card-title>
        <v-card-subtitle>Reward Cluster: {{ item.rewards.rewardCluster }}</v-card-subtitle>
        <v-divider/>
        <v-card-text>
          <v-table>
            <thead>
            <tr>
              <th>Endpoint</th>
              <th>Reward Basic</th>
              <th>Supplement</th>
              <th>Reward Finally</th>
            </tr>
            </thead>
            <tbody>
            <tr v-for="(item, ix) in item.rewards.rewardProviders" :key="`reward-cluster-provider-${item.endpoint}`">
              <td>
                <span>
                  {{ item.endpoint }}
                   <v-tooltip activator="parent" location="top">
                     {{ item.hostname }}
                   </v-tooltip>
                </span>
              </td>
              <td>{{ item.rewardGainBasic }}</td>
              <td>{{ item.rewardCompensate }}</td>
              <td>{{ item.rewardGainFinally }}</td>
            </tr>
            </tbody>
          </v-table>
        </v-card-text>
      </v-card>
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>

import {onBeforeMount, ref} from "vue";
import moment from 'moment'
import axios from 'axios'
import Stream from 'streamjs';

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
const rewardReview = ref({});
const rewardTotal = 3000;
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
    calculateReward();
  } catch (e) {
    clusterInfos.value = [];
    rewardReview.value = {};
  }
}

function calculateReward() {
  const totalParticipant = clusterInfos.value.length;
  const clusters = Stream(clusterInfos.value).groupBy(item => item.name);
  const rewardMonth = rewardTotal / 3;
  const rewardReversed = rewardMonth * 0.1;
  let rewardAvailable = rewardMonth - rewardReversed;
  const clusterNames = Object.keys(clusters);
  const rewardClusters = [];
  for (const name of clusterNames) {
    const participantCount = clusters[name].length;
    const rewardCluster = rewardAvailable * (participantCount / totalParticipant);
    const calculatedClusterReward = calculateRewardCluster(clusters[name], rewardCluster);
    rewardClusters.push({
      name,
      rewards: calculatedClusterReward,
    });
  }

  rewardReview.value = {
    rewardTotal,
    rewardMonth,
    rewardReversed,
    rewardAvailable,
    rewardClusters,
  };
}

function calculateRewardCluster(clusters: ClusterInfo[], rewardCluster: number) {
  const totalRqSuccess = Stream(clusters).sum('rq_success');
  // const totalRqError = Stream(clusters).sum('rq_error');
  // const totalRqTimeout = Stream(clusters).sum('rq_timeout');
  const rewardProviders = [];
  const rewardReview = {
    rewardCluster,
    rewardProviders,
  };
  let totalSlash = 0;
  for (const item of clusters) {
    const rateRqSuccess = item.rq_success / totalRqSuccess;
    // const rateRqError = item.rq_error / totalRqError;
    // const rateRqTimeout = item.rq_timeout / totalRqTimeout;
    const rateRqError = item.rq_error / item.rq_success;
    const rewardSuccess = rateRqSuccess * rewardCluster;
    const slashAvailable = rewardSuccess / 1;
    const slashError = slashAvailable * rateRqError;
    const rewardGainBasic = rewardSuccess - slashError;
    totalSlash += slashError;
    rewardProviders.push({
      endpoint: item.endpoint,
      hostname: item.hostname,
      rateRqSuccess,
      rateRqError,
      rewardSuccess,
      slashAvailable,
      slashError,
      rewardGainBasic,
    });
  }
  for (const item of clusters) {
    const rateRqSuccess = item.rq_success / totalRqSuccess;
    const rewardCompensate = totalSlash * rateRqSuccess;
    for (const provider of rewardProviders) {
      if (provider.endpoint !== item.endpoint) continue;
      provider.rewardCompensate = rewardCompensate;
      provider.rewardGainFinally = provider.rewardGainBasic + rewardCompensate;
    }
  }
  return rewardReview;
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

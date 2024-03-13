<script setup>
import { ref, onMounted, watch } from "vue";
import * as echarts from "echarts";
import WelcomeItem from "./WelcomeItem.vue";

const props = defineProps({
  receivedMessage: Object,
});
const error_rate = ref("0%");
const rps = ref("0.00");
const total_requests = ref(0);
const chartRPS = ref(null);
const chartResponseTime = ref(null);
const rpsData = ref([]);
const medianData = ref([]);
const ninetyFifthData = ref([]);

const legendSelectedRPS = {
  rps: true,
};
const legendSelectedResponseTime = {
  "Median Response Time": true,
  "95th Percentile Response Time": true,
};
let rpsChart;
let responseTimeChart;

const updateRPSChart = () => {
  if (rpsChart) {
    const series = [
      {
        name: "RPS",
        type: "line",
        data: rpsData.value.map((data) => [data.timestamp, data.value]),
      },
    ];

    // 添加api_results 的 RPS 数据系列
    props.receivedMessage.api_results.forEach((apiResult) => {
      series.push({
        name: `${apiResult.name} RPS`,
        type: "line",
        data: rpsData.value
          .map((data) => {
            return { value: apiResult.rps, timestamp: data.timestamp };
          })
          .map((data) => [data.timestamp, data.value]),
        showInLegend: true,
      });
    });
    props.receivedMessage.api_results.forEach((apiResult) => {
      legendSelectedRPS[`${apiResult.name} RPS`] = false; // 初始不选中
    });
    rpsChart.setOption({
      xAxis: {
        data: rpsData.value.map((data) => data.timestamp),
      },
      legend: { data: series.map((s) => s.name), selected: legendSelectedRPS },
      series: series,
    });
  }
};

const updateResponseTimeChart = () => {
  if (responseTimeChart) {
    const series = [
      {
        name: "Median Response Time",
        type: "line",
        data: medianData.value.map((data) => data.value),
      },
      {
        name: "95th Percentile Response Time",
        type: "line",
        data: ninetyFifthData.value.map((data) => data.value),
      },
    ];
    props.receivedMessage.api_results.forEach((apiResult) => {
      series.push(
        {
          name: `${apiResult.name} Median Response Time`,
          type: "line",
          data: medianData.value
            .map((data) => {
              return {
                value: apiResult.median_response_time,
                timestamp: data.timestamp,
              };
            })
            .map((data) => [data.timestamp, data.value]),
          showInLegend: true,
        },
        {
          name: `${apiResult.name} 95th Percentile Response Time`,
          type: "line",
          data: ninetyFifthData.value
            .map((data) => {
              return {
                value: apiResult.response_time_95,
                timestamp: data.timestamp,
              };
            })
            .map((data) => [data.timestamp, data.value]),
          showInLegend: true,
        }
      );
    });
    props.receivedMessage.api_results.forEach((apiResult) => {
      legendSelectedResponseTime[
        `${apiResult.name} Median Response Time`
      ] = false; // 初始不选中
      legendSelectedResponseTime[
        `${apiResult.name} 95th Percentile Response Time`
      ] = false; // 初始不选中
    });

    responseTimeChart.setOption({
      xAxis: {
        data: medianData.value.map((data) => data.timestamp),
      },
      legend: {
        data: series.map((s) => s.name),
        selected: legendSelectedResponseTime,
      },

      series: series,
    });
  }
};

onMounted(() => {
  rpsChart = echarts.init(chartRPS.value);
  responseTimeChart = echarts.init(chartResponseTime.value);

  rpsChart.setOption({
    tooltip: { trigger: "axis" },
    legend: { data: ["RPS"] },
    xAxis: { type: "category", boundaryGap: false, data: [] },
    yAxis: { type: "value" },
    series: [{ name: "RPS", type: "line", data: [] }],
  });

  responseTimeChart.setOption({
    tooltip: { trigger: "axis" },
    legend: { data: ["Median Response Time", "95th Percentile Response Time"] },
    xAxis: { type: "category", boundaryGap: false, data: [] },
    yAxis: { type: "value" },
    series: [
      { name: "Median Response Time", type: "line", data: [] },
      { name: "95th Percentile Response Time", type: "line", data: [] },
    ],
  });
});

watch(
  () => props.receivedMessage,
  (newVal) => {
    if (newVal) {
      const newTimestamp = new Date(newVal.timestamp).toLocaleTimeString();
      rpsData.value.push({ timestamp: newTimestamp, value: newVal.rps });

      // if (rpsData.value.length > 20) rpsData.value.shift();
      updateRPSChart();
      error_rate.value = newVal.error_rate.toFixed(2) + "%";
      rps.value = newVal.rps.toFixed(2) ;
      total_requests.value = newVal.total_requests.toFixed(0);
      medianData.value.push({
        timestamp: newTimestamp,
        value: newVal.median_response_time,
      });
      ninetyFifthData.value.push({
        timestamp: newTimestamp,
        value: newVal.response_time_95,
      });
      // if (medianData.value.length > 20) medianData.value.shift();
      // if (ninetyFifthData.value.length > 20) ninetyFifthData.value.shift();
      updateResponseTimeChart();
    }
  },
  { deep: true }
);
</script>

<template>
  <WelcomeItem>
    <!-- <template #heading>错误率 </template> -->
    <div
      style="
        width: 200px;
        height: 14rem;
        display: flex;
        flex-direction: column; /* 子元素垂直排列 */
        align-items: center; /* 水平居中 */
        text-align: center;
        padding-top: 2rem;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.12), 0 0 6px rgba(0, 0, 0, 0.04);
      "
    >
      <div class="flex-item">
        <el-statistic title="RPS" :value="rps" />
      </div>
      <div class="flex-row">
        <el-statistic title="错误率" :value="error_rate" class="flex-item" />
        <el-statistic
          title="总请求数据量"
          :value="total_requests"
          class="flex-item"
        />
      </div>
    </div>
  </WelcomeItem>
  <WelcomeItem>
    <template #heading>RPS</template>
    <div ref="chartRPS" style="width: 600px; height: 265px"></div>
  </WelcomeItem>
  <WelcomeItem>
    <template #heading>响应时间</template>
    <div ref="chartResponseTime" style="width: 600px; height: 265px"></div>
  </WelcomeItem>
</template>
<style scoped>
.flex-row {
  display: flex;
  width: 100%; /* 确保内部容器宽度与外部容器相同 */
  justify-content: space-around; /* 第二行的元素水平分布 */
}
.flex-item {
  margin: 10px; /* 为元素提供一些空间 */
}
</style>

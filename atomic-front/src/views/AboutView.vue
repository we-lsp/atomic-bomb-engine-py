<template>
  <div class="about">
    <div ref="chart" style="width: 700px; height: 550px"></div>
  </div>
</template>

<script setup>
import { onMounted, ref } from "vue";
import * as echarts from "echarts";
import "echarts-gl";

const chart = ref(null);

onMounted(() => {
  // 基于准备好的dom，初始化echarts实例
  const myChart = echarts.init(chart.value);

  // 指定图表的配置项和数据
  const option = {
    grid3D: {},
    xAxis3D: {
      type: "category",
    },
    yAxis3D: {},
    zAxis3D: {},
    series: [
      {
        type: "line3D",
        data: [
          [0, 0, 0],
          [1, 2, 3],
          [2, 3, 5],
          [3, 5, 8],
          [4, 8, 13],
        ],
        lineStyle: {
          width: 4, // 设置线宽
        },
      },
    ],
  };

  // 使用刚指定的配置项和数据显示图表。
  myChart.setOption(option);
});
</script>

<style>
@media (min-width: 1024px) {
  .about {
    min-height: 100vh;
    display: flex;
    align-items: center;
  }
}
</style>

<script setup>
import TheWelcome from "../components/TheWelcome.vue";
import { RouterLink, RouterView } from "vue-router";

import HelloWorld from "../components/HelloWorld.vue";

import { onMounted, ref, onUnmounted } from "vue";
import axios from "axios";

const message = ref("");
const httpIsError = ref(false);
const assertIsError = ref(false);
const buttonShow = ref(true);
const httpData = ref([]);
const assertData = ref([]);
const api_resultsData = ref([]);
const hostname = window.location.hostname; // 获取当前页面的域名或IP地址
const port = window.location.port; // 获取当前页面的端口号
const baseURL = `${hostname}${port ? ":" + port : ""}`; // 拼接域名和端口号
const ws = new WebSocket(`ws://${baseURL}/ws`);
onMounted(() => {
  ws.onopen = () => {
    console.log("WebSocket connection opened");
  };

  ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log("Message received: ", data);
    message.value = data;
    httpData.value = data.http_errors;
    assertData.value = data.assert_errors;
    api_resultsData.value = data.api_results;
    if (httpData.value.length) {
      httpIsError.value = true;
    } else {
      httpIsError.value = false;
    }
    if (assertData.value.length) {
      assertIsError.value = true;
    } else {
      assertIsError.value = false;
    }
  };

  ws.onerror = (error) => {
    console.error("WebSocket error: ", error);
  };

  ws.onclose = () => {
    console.log("WebSocket connection closed");
  };
});
onUnmounted(() => {
  ws.close(); // 关闭 WebSocket 连接
});
const sendMessage = () => {
  if (ws.readyState === WebSocket.OPEN) {
    ws.send("Hello, server!");
  } else {
    console.log("WebSocket is not open.");
  }
};
const run = async () => {
  // message.value = [];
  // httpData.value = [];
  // assertData.value = [];
  // api_resultsData.value = [];
  if (buttonShow.value) {
    const response = await axios.get(`http://${baseURL}/run`);
    console.log("response", response.data);
    message.value = response.data;
    httpData.value = response.data.http_errors;
    assertData.value = response.data.assert_errors;
    api_resultsData.value = response.data.api_results;
    buttonShow.value = false;
  }

  // buttonShow.value = true;
  // loading.value = true;
};
</script>

<template>
  <header>
    <img
      alt="Vue logo"
      class="logo"
      src="@/assets/atomic-bomb-engine-logo.png"
      width="125"
      height="125"
    />

    <div class="wrapper">
      <HelloWorld msg="压测报告" />

      <!-- <nav>
        <RouterLink to="/">2D</RouterLink>
        <RouterLink to="/about">3D</RouterLink>
      </nav> -->

      <div class="login-box" @click="run">
        <a href="#" :class="{ 'is-disabled': !buttonShow }">
          <span></span>
          <span></span>
          <span></span>
          <span></span>
          {{ buttonShow ? "开始" : "停止" }}
        </a>
      </div>
    </div>
    <main>
      <TheWelcome :receivedMessage="message" />
    </main>
    <div v-if="httpIsError">
      <h3>HTTP错误</h3>
      <el-table :data="httpData" style="width: 90rem" height="270">
        <el-table-column prop="code" label="code" width="180" />
        <el-table-column prop="message" label="message" width="180" />
        <el-table-column prop="count" label="count" />
        <el-table-column prop="url" label="url" />
      </el-table>
    </div>
    <div v-if="assertIsError">
      <h3>断言错误</h3>
      <el-table
        :data="assertData"
        style="width: 90rem; margin-top: 1rem"
        height="270"
      >
        <el-table-column prop="message" label="message" width="180" />
        <el-table-column prop="count" label="count" width="180" />
        <el-table-column prop="url" label="url" />
      </el-table>
    </div>
    <h3>接口详情</h3>
    <el-table
      :data="api_resultsData"
      style="width: 90rem; margin-top: 1rem"
      height="270"
    >
      <el-table-column prop="name" label="名称" />
      <el-table-column prop="url" label="url" />
      <el-table-column prop="method" label="请求方法" />
      <el-table-column prop="total_requests" label="总请求数" />
      <el-table-column prop="err_count" label="错误数量" />
      <el-table-column prop="error_rate" label="错误率" />
      <el-table-column prop="min_response_time" label="最小响应时间" />
      <el-table-column prop="max_response_time" label="最大响应时间" />
      <el-table-column prop="median_response_time" label="中位响应时间" />
      <el-table-column prop="response_time_95" label="95位响应时间" />
      <el-table-column prop="response_time_99" label="99位响应时间" />
      <el-table-column prop="rps" label="rps" />
      <el-table-column
        prop="throughput_per_second_kb"
        label="每秒响应数据（kb）"
      />
      <el-table-column prop="total_data_kb" label="总响应数据量（kb）" />
    </el-table>
    <footer>
      powered by
      <a
        href="https://github.com/qyzhg/atomic-bomb-engine-py?tab=readme-ov-file"
        target="_blank"
        rel="noopener"
        >atomic-bomb-engine</a
      >.
    </footer>
  </header>
</template>
<style scoped>
.is-disabled {
  background-color: grey !important;
  cursor: not-allowed;
}
.login-box {
  position: absolute;
  top: 0px;
  right: 4rem;
}
.login-box a {
  position: relative;
  display: inline-block;
  padding: 10px 20px;
  color: #fff;
  font-size: 16px;
  text-decoration: none;
  text-transform: uppercase;
  overflow: hidden;
  transition: 0.5s;
  margin-top: 40px;
  letter-spacing: 4px;
  background: #1ab188;
  /* background: #03e9f4; */
}

.login-box a:hover {
  background: #03e9f4;
  color: #fff;
  border-radius: 5px;
  box-shadow: 0 0 5px #03e9f4, 0 0 25px #03e9f4, 0 0 50px #03e9f4,
    0 0 100px #03e9f4;
}

.login-box a span {
  position: absolute;
  display: block;
}

.login-box a span:nth-child(1) {
  top: 0;
  left: -100%;
  width: 100%;
  height: 2px;
  background: linear-gradient(90deg, transparent, #03e9f4);
  animation: btn-anim1 1s linear infinite;
}

@keyframes btn-anim1 {
  0% {
    left: -100%;
  }
  50%,
  100% {
    left: 100%;
  }
}

.login-box a span:nth-child(2) {
  top: -100%;
  right: 0;
  width: 2px;
  height: 100%;
  background: linear-gradient(180deg, transparent, #03e9f4);
  animation: btn-anim2 1s linear infinite;
  animation-delay: 0.25s;
}

@keyframes btn-anim2 {
  0% {
    top: -100%;
  }
  50%,
  100% {
    top: 100%;
  }
}

.login-box a span:nth-child(3) {
  bottom: 0;
  right: -100%;
  width: 100%;
  height: 2px;
  background: linear-gradient(270deg, transparent, #03e9f4);
  animation: btn-anim3 1s linear infinite;
  animation-delay: 0.5s;
}

@keyframes btn-anim3 {
  0% {
    right: -100%;
  }
  50%,
  100% {
    right: 100%;
  }
}

.login-box a span:nth-child(4) {
  bottom: -100%;
  left: 0;
  width: 2px;
  height: 100%;
  background: linear-gradient(360deg, transparent, #03e9f4);
  animation: btn-anim4 1s linear infinite;
  animation-delay: 0.75s;
}

@keyframes btn-anim4 {
  0% {
    bottom: -100%;
  }
  50%,
  100% {
    bottom: 100%;
  }
}

main {
  display: flex;
}
header {
  padding-top: 10px;
  line-height: 1.5;
  max-height: 100vh;
}
footer {
  text-align: center;
}
.logo {
  display: block;
  margin: 0 auto 2rem;
}

nav {
  width: 100%;
  font-size: 12px;
  text-align: center;
  margin-top: 0rem;
}

nav a.router-link-exact-active {
  color: var(--color-text);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
}

nav a:first-of-type {
  border: 0;
}

@media (min-width: 1024px) {
  header {
    /* display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);*/
  }

  .logo {
    margin: 0 2rem 0 0;

    display: inline-block;
  }

  header .wrapper {
    /* display: flex;
    place-items: flex-start; */
    flex-wrap: wrap;
    display: inline-block;
  }

  nav {
    text-align: left;
    margin-left: -0.5rem;
    font-size: 1rem;
    padding: 1rem 0;
    margin-top: 0rem;
  }
}
</style>

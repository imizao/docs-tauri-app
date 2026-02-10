<script setup>
import { NConfigProvider, NMessageProvider } from "naive-ui";
import MainContent from "./MainContent.vue";
</script>

<template>
  <NConfigProvider>
    <NMessageProvider>
      <MainContent />
    </NMessageProvider>
  </NConfigProvider>
</template>

<style>
* {
  box-sizing: border-box;
}
html,
body {
  margin: 0;
  font-family: "Noto Sans SC", -apple-system, BlinkMacSystemFont, sans-serif;
  background: #f0f2f5;
  color: #1f2937;
}
#app {
  min-height: 100vh;
  padding: 16px;
}
</style>

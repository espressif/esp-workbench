<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import PathSelector from "./components/PathSelector.vue";
import PackagerOptions from "./components/PackagerOptions.vue";
</script>

<template>
  <div class="container">
    <h1>ESP Development Environment Packager</h1>
    <PackagerOptions
      esp-idf-path="/home/georgik/projects/esp-idf"
      esp-tools-path="/home/georgik/.espressif"
      output-archive="/home/georgik/esp-idf.tar.gz"
    />

  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>

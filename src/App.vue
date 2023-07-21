<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import PathSelector from "./components/PathSelector.vue";
</script>

<template>
  <div class="container">
    <h1>ESP Development Environment Packager</h1>

    <PathSelector title="ESP Tools path" path="/Users/georgik/.espressif"/>
    <PathSelector title="ESP-IDF path" path="/Users/georgik/projects/esp-idf"/>
    <PathSelector title="Output archive" path="/Users/georgik/my-esp-idf.zip"/>
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

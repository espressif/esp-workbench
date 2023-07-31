<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import PackagerOptions from "./components/PackagerOptions.vue";


const espIdfPath = ref("/Users/georgik/projects/esp-idfxx");
const espToolsPath = ref("/Users/georgik/.espressif");
const outputArchive = ref("/Users/georgik/esp-dev-env.zip");

onMounted(() => {
  invoke("get_user_home").then((user_home) => {
    console.log("User home:" + user_home);
    espIdfPath.value =  user_home + "/projects/esp-idf";
    espToolsPath.value = user_home + "/.espressif/esp-tools";
    outputArchive.value = user_home + "/esp-dev-env.zip";
  }).catch((error) => {
    console.error(error);
  });
});

</script>

<template>
  <div class="container">
    <h1>ESP Development Environment Packager</h1>
    <PackagerOptions
      v-model:esp-idf-path="espIdfPath"
      v-model:esp-tools-path="espToolsPath"
      v-model:output-archive="outputArchive"
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

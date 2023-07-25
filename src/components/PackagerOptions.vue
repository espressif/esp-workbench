<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import PathSelector from "./PathSelector.vue";

const props = defineProps(['espIdfPath','espToolsPath','outputArchive']);

const espIdfPath = ref(props.espIdfPath);
const espToolsPath = ref(props.espToolsPath);
const outputArchive = ref(props.outputArchive);

// Send request to backende to perform compression
function compressPackage() {
  let espIdf = espIdfPath.value ? espIdfPath.value.toString() : "";
  let output = outputArchive.value ? outputArchive.value.toString() : "";

  invoke("compress", {sourcePath: espIdf, targetPath: output})
    .then((message) => {
      console.log(message);
    })
    .catch((error) => {
      console.error(error);
    });
}

</script>

<template>
    <PathSelector title="ESP Tools path" v-model:path="espIdfPath"/>
    <PathSelector title="ESP-IDF path" v-model:path="espToolsPath"/>
    <PathSelector title="Output archive" v-model:path="outputArchive"/>

    <button @click="compressPackage()">Log</button>
</template>
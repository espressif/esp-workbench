<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from '@tauri-apps/api/window';
import PathSelector from "./PathSelector.vue";

const props = defineProps(['espIdfPath','espToolsPath','outputArchive']);

const espIdfPath = ref(props.espIdfPath);
const espToolsPath = ref(props.espToolsPath);
const outputArchive = ref(props.outputArchive);

const buildStatus = ref("Idle");
let register = appWindow.listen('progress', ({payload}) => buildStatus.value = payload.pct);

// Send request to backende to perform compression
function compressPackage() {
  let espIdf = espIdfPath.value ? espIdfPath.value.toString() : "";
  let output = outputArchive.value ? outputArchive.value.toString() : "";

  invoke("compress", {window: appWindow, sourcePath: espIdf, targetPath: output})
    .then((message) => {
      console.log(message);
    })
    .catch((error) => {
      console.error(error);
    });
}

// Send command to backend to abort running build process
function abortBuild() {
  invoke("abort_build")
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

    <button @click="compressPackage()">Build package</button>
    <button @click="abortBuild()">Abort build</button>
    <div>Build status: {{ buildStatus }}</div>
</template>
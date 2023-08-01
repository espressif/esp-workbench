<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from '@tauri-apps/api/window';
import PathSelector from "./PathSelector.vue";

const props = defineProps({
  espIdfPath: String,
  espToolsPath: String,
  outputArchive: String
});

const buildStatus = ref("Idle");

type Payload = {
  pct: string,
}

appWindow.listen('progress', ({payload}) => buildStatus.value = (payload as Payload).pct);

function compressPackage() {
  let espIdf = props.espIdfPath;
  let output = props.outputArchive;

  invoke("compress", {window: appWindow, sourcePath: espIdf, targetPath: output})
    .then((message) => {
      console.log(message);
    })
    .catch((error) => {
      console.error(error);
    });
}

function deployPackage() {
  let espIdf = props.espIdfPath;
  let zipArchive = props.outputArchive;

  invoke("decompress", {window: appWindow, sourcePath: zipArchive, targetPath: espIdf})
    .then((message) => {
      console.log(message);
    })
    .catch((error) => {
      console.error(error);
    });
}

function runInstallScript() {
  let shellScriptPath = props.espIdfPath + "/install.sh";

  invoke("run_esp_idf_install_script", {window: appWindow, targetPath: shellScriptPath})
    .then((message) => {
      console.log(message);
    })
    .catch((error) => {
      console.error(error);
    });
}

function downloadEspIdf() {
  let output = props.outputArchive;
  invoke("download_esp_idf", {window: appWindow, version: '4.2', targetPath: output})
    .then((message) => {
      console.log(message);
    })
    .catch((error) => {
      console.error(error);
    });
}

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
  <PathSelector title="ESP-IDF path"
    :path="props.espIdfPath"
    @update:path="(value: string) => $emit('update:espIdfPath', value)"
  />
  <PathSelector title="ESP Tools path"
    :path="props.espToolsPath"
    @update:path="(value: string) => $emit('update:espToolsPath', value)"
  />
  <PathSelector title="Output archive"
    :path="props.outputArchive"
    @update:path="(value: string) => $emit('update:outputArchive', value)"
  />
  <button @click="compressPackage()">Build package</button>
  <button @click="downloadEspIdf()">Download ESP-IDF package</button>
  <button @click="deployPackage()">Deploy package</button>
  <button @click="runInstallScript()">Run ESP-IDF install script</button>
  <button @click="abortBuild()">Abort build</button>
  <div>Build status: {{ buildStatus }}</div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from '@tauri-apps/api/window';
import PathSelector from "./PathSelector.vue";
import VersionSelector from "./VersionSelector.vue";

const props = defineProps({
  espIdfPath: String,
  espIdfVersion: String,
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
  let version = props.espIdfVersion;
  invoke("download_esp_idf", {window: appWindow, version: version, targetPath: output})
    .then((message) => {
      console.log(message);
    })
    .catch((error) => {
      console.error(error);
    });
}

async function installEspIdf() {
  try {
    let output = props.outputArchive;
    let version = props.espIdfVersion;

    // Await the completion of the download
    await invoke("download_esp_idf", {window: appWindow, version: version, targetPath: output});

    // Await the completion of the decompression
    let espIdf = props.espIdfPath;
    let zipArchive = props.outputArchive;
    await invoke("decompress", {window: appWindow, sourcePath: zipArchive, targetPath: espIdf});

    // Await the completion of the script execution
    let shellScriptPath = props.espIdfPath + "/install.sh";
    await invoke("run_esp_idf_install_script", {window: appWindow, targetPath: shellScriptPath});
  } catch (error) {
    console.error(error);
  }
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
  <!-- <PathSelector title="ESP-IDF path"
    :path="props.espIdfPath"
    @update:path="(value: string) => $emit('update:espIdfPath', value)"
  /> -->
  <VersionSelector
    :selectedVersion="props.espIdfVersion"
    @update:selectedVersion="(value: string) => $emit('update:espIdfVersion', value)"
  />
  <PathSelector title="ESP Tools path"
    :path="props.espToolsPath"
    @update:path="(value: string) => $emit('update:espToolsPath', value)"
  />
  <div>ESP-IDF Path: {{ props.espIdfPath }}</div>
  <div>ESP-IDF ZIP: {{ props.outputArchive }}</div>
  <!-- <PathSelector title="Output archive"
    :path="props.outputArchive"
    @update:path="(value: string) => $emit('update:outputArchive', value)"
  /> -->
  <!-- <button @click="compressPackage()">Build package</button> -->
  <button @click="installEspIdf()">Install ESP-IDF</button>
  <!-- <button @click="downloadEspIdf()">Download ESP-IDF package</button> -->
  <!-- <button @click="deployPackage()">Deploy package</button> -->
  <!-- <button @click="runInstallScript()">Run ESP-IDF install script</button> -->
  <button @click="abortBuild()">Cancel</button>
  <div>{{ buildStatus }}</div>
</template>

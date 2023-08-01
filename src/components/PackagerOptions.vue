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

const buildStatus = ref("");
let isInstalling = ref(false);
let isAborted = ref(false);

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
  invoke("run_esp_idf_install_script", {window: appWindow, targetPath: props.espIdfPath})
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

    isInstalling.value = true;
    // Await the completion of the download
    await invoke("download_esp_idf", {window: appWindow, version: version, targetPath: output});

    // Await the completion of the decompression
    let espIdf = props.espIdfPath;
    let zipArchive = props.outputArchive;
    if (!isAborted.value) {
       await invoke("decompress", {window: appWindow, sourcePath: zipArchive, targetPath: espIdf});
    }

    if (!isAborted.value) {
      await invoke("run_esp_idf_install_script", {window: appWindow, targetPath: props.espIdfPath});
    }
  } catch (error) {
    console.error(error);
  }
  isInstalling.value = false;
  isAborted.value = false;
}


function abortBuild() {
  invoke("abort_build")
    .then((message) => {
      console.log(message);
    })
    .catch((error) => {
      console.error(error);
    });
  isInstalling.value = false;
  isAborted.value = true;
}

</script>

<template>
  <!-- <PathSelector title="ESP-IDF path"
    :path="props.espIdfPath"
    @update:path="(value: string) => $emit('update:espIdfPath', value)"
  /> -->
  <div class="packager-container">
    <VersionSelector
      :selectedVersion="props.espIdfVersion"
      @update:selectedVersion="(value: string) => $emit('update:espIdfVersion', value)"
    />
    <PathSelector title="ESP Tools path"
      :path="props.espToolsPath"
      @update:path="(value: string) => $emit('update:espToolsPath', value)"
    />
    <div>ESP-IDF Path: {{ props.espIdfPath }}</div>
    <!-- <div>ESP-IDF ZIP: {{ props.outputArchive }}</div> -->
    <!-- <PathSelector title="Output archive"
      :path="props.outputArchive"
      @update:path="(value: string) => $emit('update:outputArchive', value)"
    /> -->

    <div class="animation-container">
      <img src="../assets/esp32-c3-rust-1.svg" alt="Installation in progress..." />
      <div v-if="isInstalling" class="led"></div>
    </div>

    <div class="button-container">
      <!-- <button @click="compressPackage()">Build package</button> -->
      <button v-if="!isInstalling" @click="installEspIdf()">Install ESP-IDF</button>
      <!-- <button @click="downloadEspIdf()">Download ESP-IDF package</button> -->
      <!-- <button @click="deployPackage()">Deploy package</button> -->
      <!-- <button @click="runInstallScript()">Run ESP-IDF install script</button> -->
      <button v-if="isInstalling" @click="abortBuild()">Cancel</button>
    </div>
  </div>
  <div>{{ buildStatus }}</div>
</template>


<style scoped>
.button-container {
  justify-content: space-between;
  padding-top: 1em;
  padding-bottom: 1em;
  padding-left: 30em;
}

.packager-container {
  padding-top: 3em;
  padding-left: 30%;
  text-align: left;
}

.animation-container {
  position: relative;
  padding-top: 1em;
}

.led {
  position: absolute;
  top: 135px;
  left: 38px;
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: red;
  animation: colorchange 10s infinite;
  box-shadow: 0 0 5px 5px red; /* Initial glow color */
}

@keyframes colorchange {
  0% { background: red; box-shadow: 0 0 5px 5px red; }
  14% { background: orange; box-shadow: 0 0 5px 5px orange; }
  28% { background: yellow; box-shadow: 0 0 5px 5px yellow; }
  42% { background: lime; box-shadow: 0 0 5px 5px lime; }
  57% { background: aqua; box-shadow: 0 0 5px 5px aqua; }
  71% { background: blue; box-shadow: 0 0 5px 5px blue; }
  85% { background: purple; box-shadow: 0 0 5px 5px purple; }
  100% { background: red; box-shadow: 0 0 5px 5px red; }
}


</style>
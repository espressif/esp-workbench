<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from '@tauri-apps/api/window';
import PathSelector from "./PathSelector.vue";
import VersionSelector from "./VersionSelector.vue";
import LogConsole from './LogConsole.vue';

const props = defineProps({
  espIdfPath: String,
  espIdfVersion: String,
  espToolsPath: String,
  outputArchive: String
});

let isInstalling = ref(false);
let isAborted = ref(false);

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
  <div class="packager-container">
    <VersionSelector
      :selectedVersion="props.espIdfVersion"
      @update:selectedVersion="(value: string) => $emit('update:espIdfVersion', value)"
    />
    <PathSelector title="ESP Tools path"
      :path="props.espToolsPath"
      :allowDirectories="true"
      @update:path="(value: string) => $emit('update:espToolsPath', value)"
    />
    <div>ESP-IDF Path: {{ props.espIdfPath }}</div>

    <div class="progress-container">
      <div class="animation-container">
        <img class="board-image" src="../assets/esp32-c3-rust-1.svg" alt="Installation in progress..." />
        <div v-if="isInstalling" class="led"></div>
      </div>
      <div class="console-container">
        <LogConsole />
        <div class="button-container">
          <button @click="installEspIdf()" :disabled="isInstalling">Install ESP-IDF</button>
          <button @click="abortBuild()" :disabled="!isInstalling">Cancel</button>
        </div>
      </div>
    </div>
  </div>

</template>


<style scoped>

button:disabled {
  background-color: #e0e0e0;
  color: #888888;
  cursor: not-allowed;
  border: 1px solid #cccccc;
}

.packager-container {
  padding-top: 2em;
  padding-left: 10%;
  text-align: left;
}

.animation-container {
  position: relative;
  padding-top: 1em;
}

.board-image {
  transform: scale(0.75);
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


.build-status {
  flex-basis: 75%;
  background-color: #ccc;
  padding: 1em;
  margin-top: 2.5em;
  overflow: hidden;
  height: 25vh;
  white-space: pre-line;
}

.progress-container {
  display: flex;
  justify-content: space-between;
  align-items: start;
  padding-right: 5em;
}

.animation-container {
  flex-basis: 5%;
  display: flex;
  justify-content: center;
  align-items: center;
  position: relative;
}

.console-container {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}

.button-container {
  display: flex;
  justify-content: flex-end; /* align buttons to the right */
  padding-top: 1em;
  padding-bottom: 1em;
}

</style>
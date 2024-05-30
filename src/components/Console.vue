<template>
  <div>
    <div v-show="isVisible" class="console">
      <div class="console-output" ref="consoleOutput">
        <div v-for="log in logs" :key="log.id">{{ log.message }}</div>
      </div>
      <input
        type="text"
        v-model="command"
        @keyup.enter="executeCommand"
        placeholder="Enter command"
      />
    </div>
    <button class="toggle-button" @click="toggleConsole">Toggle Console</button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';

const logs = ref<{ id: number, message: string }[]>([]);
let logId = 0;
const command = ref("");
const isVisible = ref(true); // Set to true to make console visible by default
const consoleOutput = ref<HTMLElement | null>(null);

const addLog = (message: string) => {
  logs.value.push({ id: logId++, message });
  nextTick(() => {
    if (consoleOutput.value) {
      consoleOutput.value.scrollTop = consoleOutput.value.scrollHeight;
    }
  });
};

const executeCommand = () => {
  if (command.value.trim()) {
    addLog(`> ${command.value}`);
    invoke('execute_command', { command: command.value })
      .then((result) => {
        addLog(result as string);
      })
      .catch((error) => {
        addLog(`Error: ${error}`);
      });
    command.value = "";
  }
};

const toggleConsole = () => {
  isVisible.value = !isVisible.value;
  console.log("Console toggled. isVisible:", isVisible.value);
};

const handleKeyDown = (event: KeyboardEvent) => {
  console.log("Key pressed:", event.key);
  if (event.key === '`') {
    event.preventDefault();
    toggleConsole();
  }
};

onMounted(() => {
  appWindow.listen('command-log', (event) => {
    const payload = event.payload as string;
    addLog(payload);
  });

  window.addEventListener('keydown', handleKeyDown);
  console.log("Key listener added");
});

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleKeyDown);
  console.log("Key listener removed");
});
</script>

<style scoped>
.console {
  position: fixed;
  bottom: 0;
  left: 0;
  width: 100%;
  max-height: 50%;
  background-color: rgba(0, 0, 0, 0.8); /* Partially transparent background */
  color: limegreen;
  overflow: auto;
  padding: 10px;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.5);
  z-index: 9999; /* Ensure the console is above other elements */
}

.console-output {
  max-height: 80%;
  overflow-y: auto;
  text-align: left; /* Align text to the left */
}

input[type="text"] {
  width: calc(100% - 110px); /* Adjust width to avoid overlap with the button */
  padding: 5px;
  margin-top: 10px;
  background-color: black;
  color: limegreen;
  border: 1px solid limegreen;
}

.toggle-button {
  position: fixed;
  bottom: 10px;
  right: 10px;
  background-color: rgba(0, 0, 0, 0.7);
  border: 1px solid limegreen;
  color: limegreen;
  cursor: pointer;
  font-size: 1em;
  padding: 5px 10px;
  z-index: 10000; /* Ensure the button is above other elements, including the console */
}

.toggle-button:hover {
  color: white;
}
</style>

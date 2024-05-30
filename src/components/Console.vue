<template>
  <div>
    <div v-show="isVisible" class="console">
      <div class="console-output" ref="consoleOutput">
        <div v-for="log in logs" :key="log.id" v-html="log.message"></div>
      </div>
      <div class="input-container">
        <input
          type="text"
          v-model="command"
          @keyup.enter="executeCommand"
          placeholder="Enter command"
          autocorrect="off"
          autocapitalize="off"
          spellcheck="false"
        />
      </div>
    </div>
    <button class="toggle-button" @click="toggleConsole">Console</button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';
import './Console.css';

// Function to escape HTML characters
const escapeHtml = (unsafe: string) => {
  return unsafe
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
};

const logs = ref<{ id: number, message: string }[]>([]);
let logId = 0;
const command = ref("");
const isVisible = ref(false);
const consoleOutput = ref<HTMLElement | null>(null);

const addLog = (message: string) => {
  logs.value.push({ id: logId++, message: escapeHtml(message) });

  nextTick(() => {
    if (consoleOutput.value) {
      consoleOutput.value.scrollTop = consoleOutput.value.scrollHeight;
    }
  });
};

const executeCommand = () => {
  if (command.value.trim()) {
    addLog(`> ${escapeHtml(command.value)}`);
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

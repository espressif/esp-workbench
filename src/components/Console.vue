<template>
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
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick, watch } from 'vue';
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

const props = defineProps<{ isVisible: boolean }>();

const logs = ref<{ id: number, message: string }[]>([]);
let logId = 0;
const command = ref("");
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

const handleKeyDown = (event: KeyboardEvent) => {
  if (event.key === '`') {
    event.preventDefault();
    props.isVisible = !props.isVisible;
  }
};

watch(() => props.isVisible, (newValue) => {
  if (newValue) {
    window.addEventListener('keydown', handleKeyDown);
  } else {
    window.removeEventListener('keydown', handleKeyDown);
  }
});

onMounted(() => {
  appWindow.listen('command-log', (event) => {
    const payload = event.payload as string;
    addLog(payload);
  });

  if (props.isVisible) {
    window.addEventListener('keydown', handleKeyDown);
  }
});

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleKeyDown);
});
</script>

<script setup lang="ts">
import { ref, onMounted, nextTick, onBeforeUpdate, onUpdated  } from 'vue';
import { appWindow } from '@tauri-apps/api/window';
import { default as AnsiUp } from 'ansi_up';

type ConsoleEvent = {
  message: string,
}

const ansi_up = new AnsiUp();
const logOutputRef = ref<HTMLDivElement | null>(null);
const logs = ref("");

onBeforeUpdate(() => {
  // Capture current scroll position and height before the update.
  // If the current scroll position is already at the bottom, we'll scroll after the update.
  const shouldScroll = logOutputRef.value
    && logOutputRef.value.scrollTop + logOutputRef.value.clientHeight >= logOutputRef.value.scrollHeight;

  onUpdated(() => {
    // After the DOM is updated, scroll to the bottom if it was previously at the bottom
    if (shouldScroll && logOutputRef.value) {
      logOutputRef.value.scrollTop = logOutputRef.value.scrollHeight;
    }
  });
});

onMounted(async () => {
  appWindow.listen("rust-console", event => {
    const payload = event.payload as ConsoleEvent;
    console.log(payload.message);
    const htmlMessage = ansi_up.ansi_to_html(payload.message);
    logs.value += htmlMessage + "<br>";

    // Ensure the log scrolls to the bottom when a new message is received
    nextTick(() => {
      if (logOutputRef.value) {
        logOutputRef.value.scrollTop = logOutputRef.value.scrollHeight;
      }
    });
  });

});
</script>

<template>
  <div class="log-output" v-html="logs" ref="logOutputRef"></div>
</template>

<style scoped>
.log-output {
  width: 100%;
  height: 300px;
  overflow-y: scroll;
  border: 1px solid #ccc;
  padding: 8px;
  white-space: pre-wrap;  /* Preserves whitespace & line breaks */
  font-family: monospace;
  background-color: #f8f8f8;
  text-align: left;
}
</style>

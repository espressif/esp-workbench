<script setup lang="ts">
import { ref, onMounted, nextTick, onBeforeUpdate, onUpdated } from 'vue';
import { appWindow } from '@tauri-apps/api/window';
import { default as AnsiUp } from 'ansi_up';

type ConsoleEvent = {
  message: string,
}

const ansi_up = new AnsiUp();
const logOutputRef = ref<HTMLDivElement | null>(null);
const logs = ref("");

function handleConsoleMessage(message: string) {
  const htmlMessage = ansi_up.ansi_to_html(message);

  if (message.startsWith("Download progress:")) {
    const lines = logs.value.split('<br>');
    lines[lines.length - 2] = htmlMessage;  // replace the second last item (since the last item is always an empty string due to the way we append with '<br>')
    logs.value = lines.join('<br>');
  } else {
    logs.value += htmlMessage + "<br>";
  }
}

onBeforeUpdate(() => {
  const shouldScroll = logOutputRef.value
    && logOutputRef.value.scrollTop + logOutputRef.value.clientHeight >= logOutputRef.value.scrollHeight;

  onUpdated(() => {
    if (shouldScroll && logOutputRef.value) {
      logOutputRef.value.scrollTop = logOutputRef.value.scrollHeight;
    }
  });
});

onMounted(async () => {
  appWindow.listen("rust-console", event => {
    const payload = event.payload as ConsoleEvent;
    console.log(payload.message);
    handleConsoleMessage(payload.message);

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
  white-space: pre-wrap;
  font-family: monospace;
  background-color: #f8f8f8;
  text-align: left;
}

@media (prefers-color-scheme: dark) {
  .log-output {
    background-color: #989898;
  }
}
</style>

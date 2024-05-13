<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';

let isMonitoring = ref(true);
const console = ref<HTMLDivElement | null>(null);
let autoscroll = ref(true);
let logData = ref("");
let port = ref("");

type Payload = {
  pct: string,
}
onMounted(() => {
  port.value = decodeURIComponent(window.location.pathname.split("/")[2]); // assuming "/monitor/:port" route

  appWindow.listen('monitor-event', ({payload}) => {
    logData.value += (payload as Payload).pct;
    if (autoscroll.value) {
      nextTick(() => {
        const div = console.value!;
        div.scrollTo({top: div.scrollHeight, behavior: "smooth"});
      });
    }
  });
  invoke('start_monitor', { port: port.value })
    .catch((error) => {
      console.error(error);
    });
  isMonitoring.value = true;
  
});

onUnmounted(() => {
  invoke('stop_monitor')
    .catch((error) => {
      console.error(error);
    });
});

const stopMonitoring = () => {
  isMonitoring.value = false;
  invoke('stop_monitor')
    .catch((error) => {
      console.error(error);
    });
};
</script>

<template>
  <div class="monitor">
    <div class="animation-container">
      <svg width="100" height="300" xmlns="http://www.w3.org/2000/svg">
        <!-- Embedding the esp-prog image -->
        <image href="../assets/esp-prog.png" x="0" y="0" width="100" height="300"/>

        <defs>
          <filter id="glow">
              <feGaussianBlur stdDeviation="2.5" result="coloredBlur"/>
              <feMerge>
                  <feMergeNode in="coloredBlur"/>
                  <feMergeNode in="SourceGraphic"/>
              </feMerge>
          </filter>
          <radialGradient id="ledGradient" cx="50%" cy="50%" r="50%" fx="50%" fy="50%">
              <stop offset="0%" style="stop-color:#9FFF70; stop-opacity:1" />
              <stop offset="100%" style="stop-color:#4CAF50; stop-opacity:1" />
          </radialGradient>
        </defs>

        <!-- The green LED circle -->
        <circle cx="60" cy="150" r="10" fill="url(#ledGradient)" filter="url(#glow)" :class="{ active: isMonitoring }" />
      </svg>
    </div>


    <div class="log-container">
      <h2>Monitoring Port {{ port }}</h2>
      <pre class="console" ref="console">{{ logData }}</pre>
      <div class="button-container">
        <input type="checkbox" v-model="autoscroll" id="autoscroll">
        <label for="autoscroll">Autoscroll</label>
        <button @click="stopMonitoring">Stop</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.console {
  text-align: left;
  background-color: black;
  color: limegreen;
  padding: 15px;
  max-height: 350px;
  overflow-y: scroll;
  white-space: pre-wrap;       /* css-3 */
  white-space: -moz-pre-wrap;  /* Mozilla, since 1999 */
  white-space: -pre-wrap;      /* Opera 4-6 */
  white-space: -o-pre-wrap;    /* Opera 7 */
  word-wrap: break-word;       /* Internet Explorer 5.5+ */
}


.monitor {
  display: flex;
  align-items: start;
  gap: 20px;
}

.animation-container {
  flex-basis: 25%;
  padding: 10px;
}

.log-container {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}

.button-container {
  display: flex;
  justify-content: flex-end;
  padding-top: 1em;
}

.button-container label {
  display: flex;
  align-items: center;
  vertical-align: middle;
}

.button-container input[type=checkbox] {
  margin-right: 10px;
  margin-top: 15px;
}

.button-container button {
  margin-left: 10px;
}

@keyframes blink {
  0%, 100% { opacity: 0.5; }
  50% { opacity: 1; }
}

/* Default state - no LED */
circle {
  opacity: 0; /* Initially, LED is not visible */
}

/* When Monitoring is active, the LED is visible and blinking */
circle.active {
  opacity: 1;
  animation: blink 1s infinite;
}

/* Default state - no blink */
#green-led {
  animation: none;
}

/* Running state - blink */
.monitor.active #green-led {
  animation: blink 1s infinite;
}


/* Running state - blink */
circle.active.blinking {
  animation: blink 1s infinite;
}
</style>


<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

let diskSpace = ref<string[]>([]);

onMounted(() => {
  invoke('get_disk_usage')
    .then((availableSpace) => {
      diskSpace.value = availableSpace as string[];
    })
    .catch((error) => {
      console.error(error);
    });
});

</script>

<template>
  <div>
    <h2>Disk Usage</h2>
    <p>Available:
      {{ diskSpace[0] }}
    </p>
  </div>
</template>
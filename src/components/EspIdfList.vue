<script setup lang="ts">

const props = defineProps({
  versions: { type: Array as () => string[], required: true },
});

</script>

<template>
  <div>
    <h2>ESP-IDF - C/C++ framework</h2>
    <div v-if="props.versions.length > 0">
      <h3>Installed versions:</h3>
      <div v-for="version in props.versions" :key="version">
        {{ version }}
      </div>
    </div>
    <div v-else>
      <h3>No ESP-IDF installed</h3>
    </div>
  </div>
</template>

<script setup lang="ts">

const props = defineProps({
  versions: { type: Array as () => string[], required: true },
});

</script>

<template>
  <div>
    ESP-IDF
    <div v-if="props.versions.length > 0">
      <div v-for="version in props.versions" :key="version">
        {{ version }}
      </div>
    </div>
    <div v-else>
      No ESP-IDF installed
    </div>
    <button @click="$emit('addNewVersion')">+ Add ESP-IDF</button>
  </div>
</template>

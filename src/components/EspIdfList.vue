<script setup lang="ts">

const props = defineProps({
  versions: { type: Array as () => string[], required: true },
});

</script>

<template>
  <div>
    <h2>ESP-IDF - C/C++ framework</h2>
    <div class="esp-idf-list">
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
    <router-link  class="add-button" to="/esp-idf">+ Install new ESP-IDF version</router-link>
  </div>
</template>

<style scoped>
.esp-idf-list {
  margin-bottom: 1em;
}
</style>
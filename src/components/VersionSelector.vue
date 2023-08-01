<script setup lang="ts">

const props = defineProps({
  selectedVersion: String,

});

let emit = defineEmits(['update:selectedVersion']);

// let selectedVersion = ref('v5.1');
let versions = [
  'v5.1',
  'v5.0.3',
  'v4.4.5',
  'v4.3.5',
  'v4.2.5',
//   'release/v5.1',
//   'release/v5.0',
//   'release/v4.4',
//   'release/v4.3',
//   'release/v4.2',
//   'master'
];
</script>

<template>
  <div>
    <label for="version-select">ESP-IDF version:</label>
    <select :value="props.selectedVersion" id="version-select"
      @change="event => emit('update:selectedVersion', (event.target as HTMLSelectElement).value || '')">
      <option v-for="version in versions" :key="version" :value="version">
        {{ version }}
      </option>
    </select>
  </div>
</template>

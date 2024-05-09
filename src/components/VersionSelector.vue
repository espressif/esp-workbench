<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const props = defineProps({
  selectedVersion: String,

});

let emit = defineEmits(['update:selectedVersion']);

let versions = ref([]);

onMounted(() => {
  invoke("get_available_idf_versions").then((response) => {
    let versionsJson = ref([]);
    try {
      versionsJson.value = JSON.parse(response);
    } catch (error) {
      console.error(error);
    }
    versions.value = versionsJson.value.map((version) => version.tag_name).sort().reverse();
    emit('update:selectedVersion',versions.value[0]) // by default select latest version
    console.log(versions.value[0]);
  }).catch((error) => {
    console.error(error);
  });
});

</script>

<template>
  <div>
    <label for="version-select">ESP-IDF version:</label>
    <select v-if="versions.length" :value="props.selectedVersion" id="version-select"
      @change="event => emit('update:selectedVersion', (event.target as HTMLSelectElement).value || '')">
      <option v-for="version in versions" :key="version" :value="version">
        {{ version }}
      </option>
    </select>
    <span class="loading" v-else> Loading...</span>
  </div>
</template>

<style scoped>
.loading {
  padding-bottom: 4px;
  background:linear-gradient(currentColor 0 0) 0 100%/0% 3px no-repeat;
  animation:l 2s linear infinite;
}
@keyframes l {to{background-size: 100% 3px}}
</style>
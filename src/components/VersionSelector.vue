<script setup lang="ts">
import { onMounted, ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { jsonrepair } from 'jsonrepair';

const props = defineProps({
  selectedVersion: String,

});

let emit = defineEmits(['update:selectedVersion']);


interface VersionInfo {
  name: string;
  old?: boolean;
  has_targets?: boolean;
  supported_targets?: string[];
  end_of_life?: boolean;
}

interface TargetInfo {
  text: string;
  value: string;
}

// let versionsData = ref([]);
let versionsData = ref<{ VERSIONS: VersionInfo[], IDF_TARGETS: TargetInfo[] }>({ VERSIONS: [], IDF_TARGETS: [] });
let selectedTarget = ref("");
let show_old = ref(false);
let show_unsuported = ref(false);

const versions = computed(() => {
  if (versionsData.value["VERSIONS"].length === 0) {
    return [];
  }
  let filteredVersions = versionsData.value["VERSIONS"].filter((version: VersionInfo) => {
    const isLatestOrRelease = version.name.includes("latest") || version.name.includes("release");
    const isUnsuported = version.end_of_life && !show_unsuported.value;
    const isOld = version.old && !show_old.value;
    const selectedTargetFilter = !selectedTarget.value.length || (!!version.supported_targets && version.supported_targets.includes(selectedTarget.value));
    return !isLatestOrRelease && !isUnsuported && !isOld && selectedTargetFilter;
  });
  let d = filteredVersions.map((version: VersionInfo) => version.name);
  return d;
})

const targets = computed(() => {
  let empty = [{text:"latest supported versions", value:""}];
  if (versionsData.value["IDF_TARGETS"].length === 0) {
    return empty;
  }
  return empty.concat(versionsData.value["IDF_TARGETS"]);
})

const targetsOfSelectedVersion = computed(() => {
  if (versionsData.value["VERSIONS"].length === 0) {
    return [];
  }
  let version = versionsData.value["VERSIONS"].find((version: VersionInfo) => version.name === props.selectedVersion);
  if (!version) {
    return [];
  }
  return version.supported_targets || [];
})

onMounted(() => {
  invoke("get_available_idf_versions").then((response) => {
    try {
      let data = JSON.parse(jsonrepair(response as string));
      versionsData.value = data;
    } catch (error) {
      console.error(error);
    }
    emit('update:selectedVersion', versions.value[0]) // by default select latest version
  }).catch((error) => {
    console.error(error);
  });
});

</script>

<template>
  <div>
    <label for="target-select">Target:</label>
    <select v-if="targets.length" id="target-select" v-model="selectedTarget">
      <option v-for="target in targets" :key="target.value" :value="target.value">
        {{ target.text }}
      </option>
    </select>
  </div>
  <div>
    <label for="version-select">ESP-IDF version:</label>
    <select v-if="versions.length" :value="props.selectedVersion" id="version-select"
      @change="event => emit('update:selectedVersion', (event.target as HTMLSelectElement).value || '')">
      <option v-for="version in versions" :key="version" :value="version">
        {{ version }}
      </option>
    </select>
    <span class="loading" v-else> Loading...</span>
    <span v-if="targetsOfSelectedVersion"> [{{ targetsOfSelectedVersion.join(', ') }}]</span>
  </div>
  <div>
    <input type="checkbox" v-model="show_old" id="old-version-select" /><label for="old-version-select">Show old versions</label>
    <input type="checkbox" v-model="show_unsuported">Show unsuported versions</input>
  </div>
</template>

<style scoped>
label {
  margin-right: 1em;
}
div {
  margin-bottom: 5px;
}
.loading {
  padding-bottom: 4px;
  background:linear-gradient(currentColor 0 0) 0 100%/0% 3px no-repeat;
  animation:l 2s linear infinite;
}
@keyframes l {to{background-size: 100% 3px}}
</style>
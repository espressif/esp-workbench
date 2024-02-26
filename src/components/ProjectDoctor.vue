<!-- The Vue component which displays inside HTML version of Rust book located at URL: https://esp-rs.github.io/book/ -->


<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
// import { appWindow } from '@tauri-apps/api/window';
// import { useRouter } from 'vue-router';
import PathSelector from './PathSelector.vue';
let projectPath = ref("");

const requestProjectDiagnostics = () => {
  if (projectPath.value) {
    invoke('request_project_diagnostics', { projectPath: projectPath.value })
    .then((project_report) => {
    console.log("Project report:" + project_report);
  }).catch((error) => {
      console.error(error);
    });
  }

};

const updateProjectPath = (path: string) => {
  projectPath.value = path;
  requestProjectDiagnostics();
  
};

onMounted(() => {
  
});
</script>

<template>
  <div class="project-doctor">
    <div>
      <h3>Select project</h3>
      <PathSelector
        title="Select project directory"
        :path="project_path"
        :allowDirectories="true"
        @update:path="updateProjectPath"
      />
    </div>
  </div>
</template>


<style lang="css" scoped>
.project-doctor {
  width: 100%;
  height: 100%;
  flex-direction: column;
  align-items: center;
}

.book-content {
  width: 100%;
  height: 100%;
  border: none;
}
</style>

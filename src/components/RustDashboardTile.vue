<template>
  <div class="card">
    <h2>Rust Support</h2>
    <div v-if="!rustInstalled">
      <p>Nothing installed.</p>
      <button @click="installRustSupport">+ Install Rust Support</button>
    </div>
    <div v-else>
      <p>
        <strong>Xtensa Toolchain:</strong> <span v-if="xtensa">Installed</span> <span v-else>Not Installed</span>
      </p>
      <p>
        <strong>RISC-V Toolchain:</strong> <span v-if="riscv">Installed</span> <span v-else>Not Installed</span>
      </p>
      <p>
        <strong>Cargo:</strong> <span v-if="cargo">Installed</span> <span v-else>Not Installed</span>
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

let rustInstalled = ref(false);
let xtensa = ref(false);
let riscv = ref(false);
let cargo = ref(false);

onMounted(() => {
  // checkRustSupport();
});

const checkRustSupport = async () => {
  try {
    const response = await invoke('check_rust_support');
    rustInstalled.value = response.rustInstalled;
    xtensa.value = response.xtensa;
    riscv.value = response.riscv;
    cargo.value = response.cargo;
  } catch (error) {
    console.error(error);
  }
};

const installRustSupport = () => {
  invoke('install_rust_support')
    .then(() => {
      checkRustSupport();
    })
    .catch((error) => {
      console.error(error);
    });
};
</script>

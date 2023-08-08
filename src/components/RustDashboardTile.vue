<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

type RustSupportResponse = {
  xtensa: string | null;
  riscv: string | null;
  cargo: string | null;
};

let rustInstalled = ref(false);
let xtensa = ref<string | null>(null);
let riscv = ref<string | null>(null);
let cargo = ref<string | null>(null);

onMounted(() => {
  checkRustSupport();
});

const checkRustSupport = async () => {
  try {
    const response: RustSupportResponse = await invoke('check_rust_support');
    console.log(response);

    rustInstalled.value = response.cargo !== null || response.xtensa !== null || response.riscv !== null;

    xtensa.value = response.xtensa;
    riscv.value = response.riscv;
    cargo.value = response.cargo;
  } catch (error) {
    console.error(error);
  }
};
</script>

<template>
  <div class="card">
    <h2>Rust Support</h2>
    <div v-if="!rustInstalled">
      <p>Nothing installed.</p>
    </div>
    <div v-else>
      <p>
        <strong>Xtensa Toolchain:</strong> <span v-if="xtensa">{{ xtensa }}</span> <span v-else>Not Installed</span>
      </p>
      <p>
        <strong>RISC-V Toolchain:</strong> <span v-if="riscv">{{ riscv }}</span> <span v-else>Not Installed</span>
      </p>
      <p>
        <strong>Cargo:</strong> <span v-if="cargo">{{ cargo }}</span> <span v-else>Not Installed</span>
      </p>
      <router-link class="add-button" to="/rust">+ Install Rust Support</router-link>
    </div>
  </div>
</template>



<template>
  <div class="card">
    <h2>Developer Portal</h2>
    <router-link class="add-button" to="/developer-portal/read">Read</router-link>
    <router-link class="add-button" to="/developer-portal/contribute">Contribute</router-link>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.add-button {
  display: inline-block;
  padding: 10px 20px;
  color: #fff;
  background-color: #007bff;
  border-radius: 5px;
  text-decoration: none;
  transition: background-color 0.2s ease;
}

.add-button:hover {
  background-color: #0056b3;
}
</style>

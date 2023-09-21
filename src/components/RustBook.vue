<!-- The Vue component which displays inside HTML version of Rust book located at URL: https://esp-rs.github.io/book/ -->


<script setup lang="ts">
</script>

<template>
  <div class="book-reader">
    <iframe class="book-content" src="https://esp-rs.github.io/book/"></iframe>
  </div>
</template>


<style lang="css" scoped>
.book-reader {
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

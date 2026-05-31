<script setup lang="ts">
import Editor from './components/Editor.vue';
</script>

<template>
  <div class="app">
    <Editor />
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html,
body,
#app {
  height: 100%;
  overflow: hidden;
}

.app {
  height: 100%;
}
</style>

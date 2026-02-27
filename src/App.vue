<script setup lang="ts">
// import { invoke } from '@tauri-apps/api/core'

// const greetMsg = ref('')
// const name = ref('')

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke('greet', { name: name.value })
// }
</script>

<template>
  <div class="bg-linear-to-br from-gray-100 to-gray-200 min-h-screen font-sans text-dark  px-4 py-8">
    <router-view />
  </div>
</template>

<style>
html,body,#app {
  width: 100%;
  height: 100%;
}
</style>

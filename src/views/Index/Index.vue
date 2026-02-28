<script setup lang="ts">
// import { listen } from '@tauri-apps/api/event'

// import { speak } from 'tauri-plugin-tts-api'

onMounted(async () => {

})
onBeforeUnmount(async () => {

})
</script>

<template>
  <div>
    <!-- 头部 -->
    <header class="text-center mb-12">
      <h1 class="text-4xl md:text-5xl font-bold text-primary mb-4 text-shadow">
        语音软件启动器
      </h1>
      <p class="text-lg mx-auto tips">
        通过语音指令快速打开您的软件，提高工作效率。简单说出"打开XX软件"即可启动相应程序。
      </p>
    </header>
  </div>
</template>

<style lang='scss' scoped>
.tips {
  color: rgb(107, 114, 128);

}
</style>

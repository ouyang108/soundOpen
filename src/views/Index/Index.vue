<script setup lang="ts">
// import { listen } from '@tauri-apps/api/event'
import { onResult, onStateChange, requestPermission, startListening, stopListening } from 'tauri-plugin-stt-api'
import { speak } from 'tauri-plugin-tts-api'
// 定义 window 上的扩展类型
interface Window {
  sttListeners?: Array<(() => void) | { unregister: () => void }>
}
let unlisten: any
async function startVoice() {
  // 开始录音
  const perm = await requestPermission()
  if (perm.microphone !== 'granted') {
    console.error('Microphone permission required')
    return
  }
  try {
    unlisten && unlisten()
    await stopListeningAll()
    await startListening({ language: 'zh-CN' })
    console.log('开始录音成功')
  }
  catch (error) {
    console.error('开始录音失败:', error)
  }
  console.log('注册了一次监听')
  await onStateChange((event) => {
    console.log('State:', event.state) // "idle" | "listening" | "processing"
  })
  unlisten = await onResult(async (result) => {
    console.log('Recognized:', result.transcript, result.isFinal)
    if (result.isFinal) {
    // 忽略ts检查
    // @ts-expect-error 忽略ts检查
      await speak({ text: result.transcript, language: 'zh-CN' })
      // 检查关键词
      const keywords = ['打开', '关闭', '启动', '关闭']
      const isKeyword = checkoutKeywords(keywords, result.transcript)
      if (isKeyword) {
        console.log('包含关键词')
      }
      else {
        console.log('不包含关键词')
      }
    }
  })
}
// 关键词
function checkoutKeywords(textArr: string[], text: string) {
  if (textArr.includes(text)) {
    return true
  }
  return false
}
async function stopListeningAll() {
  await stopListening()
  clearAllListeners()
}
function clearAllListeners() {
  const win = window as unknown as Window
  console.log(win.sttListeners)
  if (win.sttListeners && Array.isArray(win.sttListeners)) {
    console.log(`正在清理 ${win.sttListeners.length} 个残留监听器...`)
    win.sttListeners.forEach((listener) => {
      if (typeof listener === 'function') {
        listener()
      }
      else if (listener && typeof listener.unregister === 'function') {
        listener.unregister()
      }
    })
    // 清空数组
    win.sttListeners = []
  }
}
onMounted(async () => {
  startVoice()
})
onBeforeUnmount(async () => {
  await stopListeningAll()

  unlisten && unlisten()
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
      <button @click="stopListeningAll">
        开始录音
      </button>
    </header>
  </div>
</template>

<style lang='scss' scoped>
.tips {
  color: rgb(107, 114, 128);

}
</style>

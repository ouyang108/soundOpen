<script setup lang="ts">
import type { UnlistenFn } from '@tauri-apps/api/event'
import type { SttError } from 'tauri-plugin-stt-api'
import { listen } from '@tauri-apps/api/event'
import {
  getSupportedLanguages,
  onError,
  onResult,

  startListening,
  stopListening,
  checkPermission as sttCheckPermission,

  isAvailable as sttIsAvailable,

} from 'tauri-plugin-stt-api'
import { nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'

// --- 状态定义 ---
const isListening = ref(false)
const transcript = ref('')
const partialTranscript = ref('')
const language = ref('zh-CN')
const availableLanguages = ref<any[]>([])
const interimResults = ref(true)
const continuous = ref(true)
// const maxAlternatives = ref(1)

const isAvailable = ref<boolean | null>(null)
const availabilityReason = ref<string | null>(null)
const permission = ref<any>(null)
const loading = ref(false)
const error = ref<string | null>(null)
const success = ref<string | null>(null)
const downloadProgress = ref<any>(null)

const results = ref<any[]>([])
const resultsEndRef = ref<HTMLElement | null>(null)

// --- 全局清理队列 ---
let sttListeners: Array<(() => void) | { unregister: () => void }> = []

function clearListeners() {
  sttListeners.forEach((l) => {
    if (typeof l === 'function')
      l()
    else if (l?.unregister)
      l.unregister()
  })
  sttListeners = []
}

// --- 功能函数 ---
async function checkAvailability() {
  try {
    const result = await sttIsAvailable()
    isAvailable.value = result.available
    availabilityReason.value = result.reason || null
    if (result.available) {
      showSuccess('STT 可用！')
      loadLanguages()
      checkPerm()
    }
    else {
      error.value = result.reason || 'STT 不可用'
    }
  }
  catch (err) {
    error.value = `检查可用性失败: ${err}`
    isAvailable.value = false
  }
}

async function checkPerm() {
  try {
    const perm = await sttCheckPermission()
    permission.value = perm
  }
  catch (err) {
    console.error(err)
  }
}

// async function handleRequestPermission() {
//   try {
//     const perm = await sttRequestPermission()
//     permission.value = perm
//     if (perm.microphone === 'granted')
//       showSuccess('权限已获取')
//   }
//   catch (err) {
//     error.value = `获取权限失败: ${err}`
//   }
// }

async function loadLanguages() {
  try {
    const result = await getSupportedLanguages()
    availableLanguages.value = result.languages
  }
  catch (err) {
    console.error(err)
  }
}

async function handleStartListening() {
  error.value = null
  loading.value = true
  partialTranscript.value = ''

  try {
    // 1. 清理旧监听
    clearListeners()

    // 2. 注册新监听
    const resultListener = await onResult((res) => {
      if (res.isFinal) {
        transcript.value += (transcript.value ? ' ' : '') + res.transcript
        partialTranscript.value = ''
        results.value.push({
          text: res.transcript,
          isFinal: true,
          confidence: res.confidence,
          timestamp: new Date(),
        })
      }
      else {
        partialTranscript.value = res.transcript
      }
    })

    const errorListener = await onError((err: SttError) => {
      error.value = `STT 错误: ${err.message || err.code}`
      isListening.value = false
    })

    sttListeners.push(resultListener, errorListener)

    // 3. 启动
    await startListening({
      language: language.value,
      interimResults: interimResults.value,
      continuous: continuous.value,
    })

    isListening.value = true
    showSuccess('正在听...')
  }
  catch (err) {
    error.value = `启动失败: ${err}`
    isListening.value = false
  }
  finally {
    loading.value = false
  }
}

async function handleStopListening() {
  loading.value = true
  try {
    await stopListening()
    clearListeners()
    isListening.value = false
    partialTranscript.value = ''
    showSuccess('已停止')
  }
  catch (err) {
    error.value = `停止失败: ${err}`
  }
  finally {
    loading.value = false
  }
}

function showSuccess(msg: string) {
  success.value = msg
  setTimeout(() => success.value = null, 2000)
}

// --- 生命周期 & 监听 ---
let unlistenProgress: UnlistenFn

onMounted(async () => {
  checkAvailability()

  // 监听下载进度
  unlistenProgress = await listen<any>('stt://download-progress', (event) => {
    downloadProgress.value = event.payload
    if (event.payload.status === 'complete') {
      showSuccess(`模型 ${event.payload.model} 下载完成`)
      setTimeout(() => {
        downloadProgress.value = null
        loadLanguages()
      }, 2000)
    }
  })
})

onBeforeUnmount(async () => {
  if (unlistenProgress)
    unlistenProgress()
  await handleStopListening()
})

// 自动滚动到底部
watch(results, () => {
  nextTick(() => {
    resultsEndRef.value?.scrollIntoView({ behavior: 'smooth' })
  })
}, { deep: true })
</script>

<template>
  <div class="max-w-3xl mx-auto p-4 space-y-4 font-sans text-gray-800">
    <header class="p-6 rounded-xl bg-gradient-to-br from-indigo-500 to-purple-600 text-white shadow-lg">
      <h1 class="text-2xl font-bold mb-2">
        🎤 语音转文字示例 (Vue 3)
      </h1>
      <p class="opacity-80 text-sm">
        测试 Tauri 原生 Speech-to-Text 插件功能
      </p>
    </header>

    <div v-if="error" class="p-4 bg-red-100 text-red-700 rounded-lg border border-red-200">
      {{ error }}
    </div>
    <div v-if="success" class="p-4 bg-green-100 text-green-700 rounded-lg border border-green-200">
      {{ success }}
    </div>

    <div v-if="downloadProgress" class="p-4 bg-gray-50 rounded-lg border">
      <div class="flex items-center gap-3">
        <div class="animate-spin h-5 w-5 border-2 border-indigo-500 border-t-transparent rounded-full" />
        <div class="flex-1">
          <p class="text-sm font-medium">
            {{ downloadProgress.status }} {{ downloadProgress.model }}...
          </p>
          <div class="w-full bg-gray-200 h-2 mt-2 rounded-full overflow-hidden">
            <div class="bg-green-500 h-full transition-all duration-300" :style="{ width: `${downloadProgress.progress}%` }" />
          </div>
        </div>
      </div>
    </div>

    <section class="p-4 bg-white rounded-lg shadow border space-y-4">
      <h2 class="text-lg font-bold border-b pb-2">
        配置
      </h2>

      <div>
        <label class="text-sm block mb-2 font-medium">语言选择 (✓ = 已安装)</label>
        <div class="flex flex-wrap gap-2">
          <button
            v-for="lang in availableLanguages"
            :key="lang.code"
            :disabled="isListening"
            class="px-3 py-1 rounded border text-sm transition"
            :class="[
              language === lang.code ? 'bg-indigo-600 text-white border-indigo-600' : 'bg-white hover:bg-gray-50',
              lang.installed ? 'text-green-600 font-bold' : 'text-gray-400',
            ]" @click="language = lang.code"
          >
            {{ lang.code }} {{ lang.installed ? '✓' : '↓' }}
          </button>
        </div>
      </div>

      <div class="flex gap-6">
        <label class="flex items-center gap-2 cursor-pointer">
          <input v-model="interimResults" type="checkbox" :disabled="isListening">
          <span class="text-sm">实时结果 (Interim)</span>
        </label>
        <label class="flex items-center gap-2 cursor-pointer">
          <input v-model="continuous" type="checkbox" :disabled="isListening">
          <span class="text-sm">连续识别 (Continuous)</span>
        </label>
      </div>
    </section>

    <section class="flex justify-center gap-4">
      <button
        v-if="!isListening"
        :disabled="loading || isAvailable === false"
        class="flex items-center gap-2 bg-red-500 hover:bg-red-600 text-white px-6 py-3 rounded-full font-bold shadow-md transition-all disabled:opacity-50"
        @click="handleStartListening"
      >
        <span v-if="loading" class="animate-pulse">加载中...</span>
        <template v-else>
          开始录音
        </template>
      </button>

      <button
        v-else
        class="flex items-center gap-2 bg-gray-800 hover:bg-black text-white px-6 py-3 rounded-full font-bold shadow-md transition-all"
        @click="handleStopListening"
      >
        停止录音
      </button>

      <button
        :disabled="isListening"
        class="px-6 py-3 border border-gray-300 rounded-full hover:bg-gray-50 disabled:opacity-30"
        @click="transcript = ''; results = []"
      >
        清空
      </button>
    </section>

    <section class="bg-gray-50 p-4 rounded-lg border min-h-[120px] relative">
      <div v-if="isListening" class="absolute top-2 right-2 flex items-center gap-1 text-red-500 text-xs animate-pulse">
        <div class="w-2 h-2 bg-red-500 rounded-full" /> 录音中...
      </div>
      <p class="text-lg leading-relaxed">
        {{ transcript }}
        <span class="text-gray-400 italic">{{ partialTranscript }}</span>
      </p>
      <p v-if="!transcript && !partialTranscript" class="text-gray-400 text-sm">
        点击“开始录音”在这里查看转义文字...
      </p>
    </section>

    <section v-if="results.length > 0" class="space-y-2">
      <h3 class="font-bold text-gray-500 text-sm uppercase">
        历史记录 ({{ results.length }})
      </h3>
      <div class="max-h-60 overflow-y-auto border rounded-lg divide-y bg-white">
        <div v-for="(res, i) in results" :key="i" class="p-3 text-sm">
          <div class="flex justify-between">
            <span class="font-medium text-gray-700">{{ res.text }}</span>
            <span class="text-[10px] text-gray-400">{{ res.timestamp.toLocaleTimeString() }}</span>
          </div>
          <div class="mt-1 flex gap-2">
            <span class="bg-green-100 text-green-700 px-1.5 rounded text-[10px]">Final</span>
            <span v-if="res.confidence" class="bg-blue-50 text-blue-600 px-1.5 rounded text-[10px]">
              {{ (res.confidence * 100).toFixed(0) }}% 置信度
            </span>
          </div>
        </div>
        <div ref="resultsEndRef" />
      </div>
    </section>
  </div>
</template>

<style scoped>
/* 可以在这里添加一些动画 */
button:active {
  transform: scale(0.95);
}
</style>

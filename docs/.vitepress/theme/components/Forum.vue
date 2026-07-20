<script setup lang="ts">
import { ref, computed } from 'vue'
import { useData } from 'vitepress'
import Giscus from './Giscus.vue'
import Search from './Search.vue'

const { lang } = useData()

// Logic terjemahan yang lebih rapi
const t = computed(() => {
  switch (lang.value) {
    case 'id-ID':
      return {
        placeholder: 'Cari diskusi...',
        back: 'Kembali'
      }
    case 'en-US':
    default:
      return {
        placeholder: 'Search discussions...',
        back: 'Back'
      }
  }
})

const query = ref('')
const searching = ref(false)

function submit() {
  const value = query.value.trim()
  if (!value) {
    searching.value = false
    return
  }
  searching.value = true
}

function clearSearch() {
  query.value = ''
  searching.value = false
}
</script>

<template>
  <div class="forum-container">
    <div class="search-box">
      <input
        v-model="query"
        type="search"
        :placeholder="t.placeholder"
        @keydown.enter="submit"
        class="search-input"
      />
      <button v-if="searching" @click="clearSearch" class="back-btn">
        {{ t.back }}
      </button>
    </div>

    <div class="content-area">
      <Search v-if="searching" :query="query" @close="clearSearch" />
      <Giscus v-else />
    </div>
  </div>
</template>

<!-- Style tetap sama -->
<style scoped>
.forum-container {
  padding-top: 24px;
}

.search-box {
  display: flex;
  gap: 12px;
  margin-bottom: 24px;
}

.search-input {
  width: 100%;
  padding: 8px 16px;
  border-radius: 8px;
  border: 1px solid var(--vp-input-border-color);
  background-color: var(--vp-input-bg-color);
  transition: border-color 0.25s, background-color 0.25s;
}

.search-input:hover {
  border-color: var(--vp-c-brand-1);
}

.search-input:focus {
  outline: none;
  border-color: var(--vp-c-brand-1);
}

.back-btn {
  padding: 0 16px;
  border-radius: 8px;
  background-color: var(--vp-c-brand-1);
  color: var(--vp-c-white);
  font-weight: 500;
  transition: background-color 0.25s;
}

.back-btn:hover {
  background-color: var(--vp-c-brand-2);
}

.content-area {
  min-height: 300px;
}
</style>

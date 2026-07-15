<script setup lang="ts">
import { ref } from 'vue'
import Giscus from './Giscus.vue'
import Search from './Search.vue'

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
  <div class="forum">
    <div class="search-bar">
      <input
        v-model="query"
        type="search"
        placeholder="Search discussions..."
        @keydown.enter="submit"
      >

      <button
        v-if="searching"
        @click="clearSearch"
      >
        Back
      </button>
    </div>

    <Search
      v-if="searching"
      :query="query"
      @close="clearSearch"
    />

    <Giscus
      v-else
    />
  </div>
</template>

<style scoped>
.forum {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.search-bar {
  display: flex;
  gap: 12px;
}

.search-bar input {
  flex: 1;
}
</style>
<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{
  query: string
}>()

const loading = ref(false)

const discussions = ref<
  {
    title: string
    url: string
  }[]
>([])

watch(
  () => props.query,
  async (query) => {
    if (!query.trim()) {
      discussions.value = []
      return
    }

    loading.value = true

    try {
      const res = await fetch(
        `/api/search?q=${encodeURIComponent(query)}`
      )

      discussions.value = await res.json()
    } finally {
      loading.value = false
    }
  },
  {
    immediate: true
  }
)
</script>

<template>
  <p v-if="loading">
    Searching...
  </p>

  <ul v-else>
    <li
      v-for="discussion in discussions"
      :key="discussion.url"
    >
      <a
        :href="discussion.url"
        target="_blank"
      >
        {{ discussion.title }}
      </a>
    </li>
  </ul>
</template>
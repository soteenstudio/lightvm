<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{
  query: string
}>()

const loading = ref(false)

const comments = ref<
  {
    id: string
    author: string
    preview: string
    url: string
  }[]
>([])

watch(
  () => props.query,
  async (query) => {
    if (!query.trim()) {
      comments.value = []
      return
    }

    loading.value = true

    try {
      const res = await fetch(
        `/api/search?q=${encodeURIComponent(query)}`
      )

      comments.value = await res.json()
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
      v-for="comment in comments"
      :key="comment.id"
    >
      <a
        :href="comment.url"
        target="_blank"
      >
        {{ comment.preview }}
      </a>

      <small>
        — {{ comment.author }}
      </small>
    </li>
  </ul>
</template>
<script setup lang="ts">
import Giscus from '@giscus/vue'
import { computed } from 'vue'
import { useData } from 'vitepress'

const { lang, isDark } = useData()

const giscusLang = computed(() => {
  switch (lang.value) {
    case 'id-ID':
      return 'id'
    case 'en-US':
      return 'en'
    default:
      return 'en'
  }
})

const giscusTheme = computed(() =>
  isDark.value ? 'dark' : 'light'
)
</script>

<template>
  <Giscus
    repo="soteenstudio/lightvm"
    repo-id="R_kgDORPPqfg"
    category="General"
    category-id="DIC_kwDORPPqfs4DAC7o"
    mapping="specific"
    term="forum"
    strict="0"
    reactions-enabled="1"
    emit-metadata="0"
    input-position="bottom"
    :theme="giscusTheme"
    :lang="giscusLang"
  />
</template>
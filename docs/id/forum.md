<script setup>
import { defineAsyncComponent } from 'vue'

const Forum = defineAsyncComponent(() => import('../.vitepress/theme/components/Forum.vue'))
</script>

<Forum />

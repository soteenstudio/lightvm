<script setup>
import { defineAsyncComponent } from 'vue'

const Forum = defineAsyncComponent(() => import('../.vitepress/theme/components/Forum.vue'))
</script>

# Community Forum

Browse community topics or start a discussion about using and developing LightVM.

<Forum />

For bug reports and reproducible defects, use the [LightVM issue tracker](https://github.com/soteenstudio/lightvm/issues).

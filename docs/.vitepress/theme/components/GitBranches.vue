<template>
  <div class="git-browser">
    <div class="repo-header">
      <h1>/{{ owner }}/{{ repo }}/branches</h1>
    </div>

    <div v-if="loading" class="loading">Loading branches...</div>
    
    <table v-else class="file-table">
      <thead>
        <tr>
          <th>Name</th>
          <th>Commit</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="branch in displayedBranches" :key="branch.name" class="file-row">
          <td>
            <a :href="`https://github.com/${owner}/${repo}/tree/${branch.name}`" target="_blank">{{ branch.name }}</a>
          </td>
          <td>{{ branch.commit.sha.substring(0, 7) }}</td>
        </tr>
      </tbody>
    </table>

    <!-- Tombol More -->
    <button 
      v-if="sortedBranches.length > limit" 
      @click="showMore" 
      class="more-btn"
    >
      More...
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';

const props = defineProps<{ owner: string; repo: string; }>();
const branches = ref<any[]>([]);
const loading = ref(true);
const limit = ref(5);

// Logic Pengurutan: main/master selalu di atas, sisanya A-Z
const sortedBranches = computed(() => {
  return [...branches.value].sort((a, b) => {
    const priority = ['main', 'master'];
    if (priority.includes(a.name) && !priority.includes(b.name)) return -1;
    if (!priority.includes(a.name) && priority.includes(b.name)) return 1;
    return a.name.localeCompare(b.name);
  });
});

const displayedBranches = computed(() => {
  return sortedBranches.value.slice(0, limit.value);
});

const showMore = () => {
  limit.value += 5;
};

const fetchBranches = async () => {
  loading.value = true;
  try {
    const response = await fetch(`https://api.github.com/repos/${props.owner}/${props.repo}/branches`);
    branches.value = await response.json();
  } catch (err) { 
    console.error("Gagal narik branches:", err); 
  } finally { 
    loading.value = false; 
  }
};

onMounted(() => fetchBranches());
</script>

<style scoped>
.git-browser { font-family: var(--vp-font-family-mono); padding: 20px; background-color: var(--vp-c-bg-soft); border: 1px solid var(--vp-c-divider); border-radius: 8px; transition: background-color 0.3s; }
.repo-header h1 { margin-top: 0; font-size: 1.2rem; color: var(--vp-c-text-1); }
.file-table { width: 100%; border-collapse: collapse; margin-top: 15px; }
.file-table th { text-align: left; padding: 10px; background-color: var(--vp-c-bg-mute); border-bottom: 1px solid var(--vp-c-divider); color: var(--vp-c-text-2); }
.file-table td { padding: 8px; border-bottom: 1px solid var(--vp-c-divider); color: var(--vp-c-text-1); }
.file-row:hover { background-color: var(--vp-c-bg-mute); }
a { color: var(--vp-c-brand-1); text-decoration: none; cursor: pointer; }
a:hover { text-decoration: underline; }
.loading { padding: 20px; font-style: italic; color: var(--vp-c-text-2); }

/* Tombol More */
.more-btn { 
  margin-top: 15px; 
  padding: 8px 12px; 
  background-color: var(--vp-c-bg-mute); 
  border: 1px solid var(--vp-c-divider); 
  border-radius: 4px; 
  color: var(--vp-c-brand-1); 
  cursor: pointer; 
  font-family: var(--vp-font-family-mono);
}
.more-btn:hover { background-color: var(--vp-c-divider); }
</style>

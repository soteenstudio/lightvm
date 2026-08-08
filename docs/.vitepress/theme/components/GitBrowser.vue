<template>
  <div class="git-browser">
    <div class="repo-header">
      <!-- Tombol Back untuk naik satu tingkat -->
      <button v-if="currentPath !== ''" @click="goBack" class="back-btn">.. (Back)</button>
      <h1>/{{ owner }}/{{ repo }}/{{ currentPath }}</h1>
    </div>

    <div v-if="loading" class="loading">Loading...</div>
    
    <table v-else class="file-table">
      <thead>
        <tr>
          <th>Name</th>
          <th>Type</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="file in sortedFiles" :key="file.sha" class="file-row">
          <td>
            <!-- Kalau folder, panggil fetchContent lagi. Kalau file, buka isi -->
            <a href="#" @click.prevent="handleItemClick(file)">{{ file.name }}</a>
          </td>
          <td>{{ file.type }}</td>
        </tr>
      </tbody>
    </table>

    <!-- Tampilan isi file -->
    <div v-if="fileContent" class="file-content">
      <button @click="fileContent = ''">Close File</button>
      <pre><code>{{ fileContent }}</code></pre>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';

const props = defineProps<{ owner: string; repo: string; }>();
const files = ref<any[]>([]);
const currentPath = ref('');
const fileContent = ref('');
const loading = ref(true);

// Logic Pengurutan: Folder dulu (dir), baru File, lalu keduanya urut abjad (A-Z)
const sortedFiles = computed(() => {
  return [...files.value].sort((a, b) => {
    if (a.type !== b.type) {
      return a.type === 'dir' ? -1 : 1;
    }
    return a.name.localeCompare(b.name);
  });
});

const fetchContent = async (path: string) => {
  loading.value = true;
  try {
    const response = await fetch(`https://api.github.com/repos/${props.owner}/${props.repo}/contents/${path}`);
    const data = await response.json();
    files.value = Array.isArray(data) ? data : [data]; // Kalau file tunggal, bungkus array
    currentPath.value = path;
  } catch (err) { console.error(err); } 
  finally { loading.value = false; }
};

const handleItemClick = async (file: any) => {
  if (file.type === 'dir') {
    fetchContent(file.path);
  } else {
    // Ambil isi file (GitHub API ngasih base64)
    const res = await fetch(file.url);
    const data = await res.json();
    fileContent.value = atob(data.content); // decode base64
  }
};

const goBack = () => {
  const parts = currentPath.value.split('/');
  parts.pop();
  fetchContent(parts.join('/'));
};

onMounted(() => fetchContent(''));
</script>

<style scoped>
/* CSS kamu tetap sama, saya cuma tambahin dikit buat tombol */
.git-browser { font-family: var(--vp-font-family-mono); padding: 20px; background-color: var(--vp-c-bg-soft); border: 1px solid var(--vp-c-divider); border-radius: 8px; }
.repo-header h1 { margin-top: 0; font-size: 1.2rem; color: var(--vp-c-text-1); }
.back-btn { cursor: pointer; background: none; border: none; color: var(--vp-c-brand-1); margin-bottom: 5px; }
.file-table { width: 100%; border-collapse: collapse; margin-top: 15px; }
.file-table th { text-align: left; padding: 10px; background-color: var(--vp-c-bg-mute); border-bottom: 1px solid var(--vp-c-divider); color: var(--vp-c-text-2); }
.file-table td { padding: 8px; border-bottom: 1px solid var(--vp-c-divider); color: var(--vp-c-text-1); }
.file-row:hover { background-color: var(--vp-c-bg-mute); }
a { color: var(--vp-c-brand-1); text-decoration: none; cursor: pointer; }
a:hover { text-decoration: underline; }
.file-content { margin-top: 20px; background: #000; color: #fff; padding: 15px; overflow: auto; }
</style>

<template>
  <div class="hero-stats">
    <p v-if="loading">Syncing stats...</p>
    <p v-else-if="error">Stats unavailable</p>
    <p v-else> 
      {{ stats.weekly }}k weekly downloads | 
      {{ stats.allTime }}k all-time downloads
    </p>
  </div>
</template>

<script setup>
import stats from '../../../stats.json';
</script>

<style scoped>
.hero-stats {
  margin-top: 24px;
  text-align: center;
  color: var(--vp-c-text-2);
  font-size: 0.9rem;
}
</style>

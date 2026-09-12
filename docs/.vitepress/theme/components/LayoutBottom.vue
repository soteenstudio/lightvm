<template>
  <EcosystemFooter
    :items="networkData"
    :translations="{ en: 'Network', id: 'Jaringan' }"
  />
  <EcosystemFooter
    :items="affiliatesData"
    :translations="{ en: 'Affiliates', id: 'Afiliasi' }"
  />
</template>

<script setup>
import EcosystemFooter from './EcosystemFooter.vue';
import networkData from '../../../data/network.json';
import affiliatesData from '../../../data/affiliates.json';
</script>
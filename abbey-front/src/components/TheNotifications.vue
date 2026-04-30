<script setup lang="ts">
import useNotifications from '@/composables/useNotifications';

import ANotification from '@/components/ANotification.vue';

const { allNotifications, remove } = useNotifications()
</script>

<template>
  <a-grid rows id="the-notifications">
    <a-notification v-for="notification in allNotifications" :key="`notification-${notification.id}`"
      :notification="notification" @close="remove" />
  </a-grid>
</template>

<style scoped>
#the-notifications {
  position: absolute;
  top: 0;
  right: 0;
  z-index: 10;
  padding: var(--space-2) var(--space-1);
  max-width: 20vw;
  justify-items: end;
  row-gap: var(--space-2);
}
</style>

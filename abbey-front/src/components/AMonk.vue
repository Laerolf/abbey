<script setup lang="ts">
import { computed } from 'vue'
import { storeToRefs } from 'pinia'

import { useCatalogStore } from '@/stores/catalogStore'

import useLocale from '@/composables/useLocale'

import type { MonkDto, SkillDto } from '@/api'

const props = defineProps<{ monk: MonkDto, option?: boolean, selected?: boolean }>()

const emit = defineEmits<{ (event: 'selected', id: MonkDto['id']): void }>()

const catalogStore = useCatalogStore()

const { skills: catalogSkills } = storeToRefs(catalogStore)

const { translate, translateInScope } = useLocale('components.aMonk')

const skills = computed<SkillDto[]>(() =>
  props.monk.skill_ids
    .map((skillId) => catalogSkills.value.find(({ id }) => skillId === id))
    .filter((skill): skill is SkillDto => skill !== undefined),
)

const classes = computed(() => ({
  option: props.option,
  selected: props.selected
}))

function handleClick(): void {
  if (!props.option) {
    return
  }

  emit("selected", props.monk.id)
}
</script>

<template>
  <a-card @click="handleClick" dense class="a-monk" :class="classes">
    <template #header>
      <h5>{{ monk.name }}</h5>
    </template>

    <a-grid class="skills">
      <span class="skills-label">{{ translateInScope('skills') }}</span>

      <a-grid rows class="skills-content">
        <span v-for="skill in skills" :key="`monk-${monk.id}-skill-${skill.id}`">
          {{ translate(`catalog.skills.${skill.name}`) }}
        </span>
      </a-grid>
    </a-grid>
  </a-card>
</template>

<style scoped>
.a-monk {
  border-width: var(--border-width-1);

  &.option {
    cursor: pointer;
  }

  &.selected {
    border-color: var(--color-ui-hover);
  }

  .skills-label {
    font-weight: bold;
  }

  .skills-content {
    grid-template-columns: repeat(2, 1fr);
  }
}
</style>

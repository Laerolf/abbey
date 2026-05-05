<script setup lang="ts">
import { computed } from 'vue'
import { storeToRefs } from 'pinia';

import { useCatalogStore } from '@/stores/catalogStore';

import useLocale from '@/composables/useLocale'

import type { MonkDto, SkillDto } from '@/api'


const props = defineProps<{ monk: MonkDto }>()

const catalogStore = useCatalogStore()

const { skills: catalogSkills } = storeToRefs(catalogStore)

const { translate, translateInScope } = useLocale('components.aMonk')

const skills = computed<SkillDto[]>(() => props.monk.skill_ids.map(skillId => catalogSkills.value.find(({ id }) => skillId === id)).filter((skill): skill is SkillDto => skill !== undefined))
</script>

<template>
  <a-card class="a-monk">
    <template #header>
      <h4>{{ monk.name }}</h4>
    </template>

    <a-grid class="skills" rows>
      <h5>{{ translateInScope('skills') }}</h5>

      <ul>
        <li v-for="skill in skills" :key="`monk-${monk.id}-skill-${skill.id}`">{{
          translate(`catalog.skills.${skill.name}`)
        }}
        </li>
      </ul>
    </a-grid>
  </a-card>
</template>

<style scoped>
.skills {
  row-gap: 0;
}
</style>

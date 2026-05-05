import { computed, ref } from "vue";
import { defineStore } from "pinia";

import { getAllResources, getAllSkills } from "@/api";

import useLogger from "@/composables/useLogger";

import type { ResourceDto, SkillDto } from "@/api";

const LOG_SCOPE = '@/stores/catalogStore.ts'

export const useCatalogStore = defineStore("catalog", () => {

  const skills = ref<SkillDto[]>([])
  const resources = ref<ResourceDto[]>([])

  const isTheCatalogLoaded = computed<boolean>(() => !!skills.value.length && !!resources.value.length)

  /**
   * Loads the Catalog.
   */
  async function loadTheCatalog(): Promise<void> {
    const logger = useLogger(LOG_SCOPE)

    if (isTheCatalogLoaded.value) {
      logger.debug("The Catalog is already loaded, skipping this request :P")
      return
    }

    try {
      const [skillCatalogResponse, resourceCatalogResponse] = await Promise.all([getAllSkills(), getAllResources()])

      if (skillCatalogResponse.error) throw skillCatalogResponse.error
      if (resourceCatalogResponse.error) throw resourceCatalogResponse.error


      skills.value = skillCatalogResponse.data ?? []
      resources.value = resourceCatalogResponse.data ?? []
    } catch (error) {
      logger.error("Failed to load the Catalog", error)
      throw error
    }
  }

  return {
    skills: computed<SkillDto[]>(() => skills.value),
    resources: computed<ResourceDto[]>(() => resources.value),
    loadTheCatalog
  }
})

import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'

import AForm from '@/components/global/input/AForm.vue'

describe('AForm', () => {
  it('renders properly', () => {
    // When
    const aForm = mount(AForm)

    // Then
    expect(aForm.classes()).toStrictEqual(['a-form'])

    const aFormActions = aForm.find('.actions')
    expect(aFormActions.exists()).toBeTruthy()
  })
})

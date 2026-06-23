import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'

import AButton from '@/components/global/structure/AButton.vue'

describe('AButton', () => {
  it('renders properly', () => {
    // Given
    const expectedText = 'What a button!'

    // When
    const aButton = mount(AButton, { slots: { default: expectedText } })

    // Then
    expect(aButton.classes()).toStrictEqual(['a-button'])
    expect(aButton.text()).toContain(expectedText)
  })
})

import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'

import ATextField from '@/components/global/input/ATextField.vue'

describe('ATextField', () => {
  it('renders properly', () => {
    // Given
    const expectedLabelText = 'Does a pony crap in the woods?'
    const expectedFieldName = 'ponyCrap'

    // When
    const aTextField = mount(ATextField, {
      props: { name: expectedFieldName },
      slots: { default: expectedLabelText },
    })

    // Then
    expect(aTextField.classes()).toStrictEqual(['a-grid', 'rows', 'a-text-field'])

    const label = aTextField.find('label')
    expect(label.text()).toBe(expectedLabelText)

    const input = aTextField.find('input')
    expect(input.attributes('name')).toBe(expectedFieldName)
  })
})

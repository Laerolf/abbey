import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'

import ASelectField from '@/components/global/input/ASelectField.vue'

describe('ASelectField', () => {
  it('renders properly', () => {
    // Given
    const expectedLabelText = 'Does a pony crap in the woods?'
    const expectedFieldName = 'ponyCrap'
    const expectedOptions = { yes: 'Yes', no: 'No' }

    // When
    const aSelectField = mount(ASelectField, {
      props: { name: expectedFieldName, options: expectedOptions },
      slots: { default: expectedLabelText },
    })

    // Then
    expect(aSelectField.classes()).toStrictEqual(['a-grid', 'rows', 'a-select-field'])

    const label = aSelectField.find('label')
    expect(label.text()).toBe(expectedLabelText)

    const input = aSelectField.find('select')
    expect(input.attributes('name')).toBe(expectedFieldName)
  })
})

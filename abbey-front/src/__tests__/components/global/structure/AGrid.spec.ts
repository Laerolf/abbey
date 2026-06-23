import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'

import AGrid from '@/components/global/structure/AGrid.vue'

describe('AGrid', () => {
  it('renders properly', () => {
    // Given
    const expectedText = 'What a grid!'

    // When
    const aGrid = mount(AGrid, { slots: { default: expectedText } })

    // Then
    expect(aGrid.classes()).toStrictEqual(['a-grid', 'columns'])
    expect(aGrid.text()).toContain(expectedText)
  })

  it('renders with a row grid flow when the "rows" prop is provided', () => {
    // When
    const aGrid = mount(AGrid, { props: { rows: true } })

    // Then
    expect(aGrid.classes()).toStrictEqual(['a-grid', 'rows'])
  })
})

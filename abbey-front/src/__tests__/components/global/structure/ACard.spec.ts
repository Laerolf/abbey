import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'

import ACard from '@/components/global/structure/ACard.vue'

describe('ACard', () => {
  it('renders properly', () => {
    // Given
    const expectedHeaderText = "I'm the header!"
    const expectedBodyText = 'What a card!'
    const expectedFooterText = "I'm the footer!"

    // When
    const aCard = mount(ACard, {
      slots: { header: expectedHeaderText, default: expectedBodyText, footer: expectedFooterText },
    })

    // Then
    expect(aCard.classes()).toStrictEqual(['a-grid', 'rows', 'a-card'])

    const header = aCard.find('.a-card-header')
    expect(header.text()).toBe(expectedHeaderText)

    const body = aCard.find('.a-card-body')
    expect(body.text()).toBe(expectedBodyText)

    const footer = aCard.find('.a-card-footer')
    expect(footer.text()).toBe(expectedFooterText)
  })

  it('renders only the header and the footer when needed', () => {
    // Given
    const expectedBodyText = 'What a card!'

    // When
    const aCard = mount(ACard, { slots: { default: expectedBodyText } })

    // Then
    expect(aCard.classes()).toStrictEqual(['a-grid', 'rows', 'a-card'])

    const header = aCard.find('.a-card-header')
    expect(header.exists()).toBeFalsy()

    const body = aCard.find('.a-card-body')
    expect(body.text()).toBe(expectedBodyText)

    const footer = aCard.find('.a-card-footer')
    expect(footer.exists()).toBeFalsy()
  })
})

/**
 *
 * @param {import('html-webpack-plugin').HtmlTagObject} tag
 */
function openTag(tag) {
  return `<${tag.tagName} ${renderAttributes(tag.attributes)}`
}

function renderAttributes(attributesMap) {
  const attributes = []
  for (const key of Reflect.ownKeys(attributesMap)) {
    if (typeof key === 'symbol') continue
    const value = Reflect.get(attributesMap, key)
    const valueType = typeof value
    switch (valueType) {
      case 'boolean':
        if (value) {
          attributes.push(key)
        }
        break
      case 'number':
      case 'bigint':
      case 'string':
        if (valueType === 'string' && !value) continue
        attributes.push(`${key}="${value}"`.replace('\\', ''))
        break
    }
  }

  return attributes.join(' ')
}

/**
 *
 * @param {import('html-webpack-plugin').HtmlTagObject} tag
 */
function closeTag(tag) {
  return tag.voidTag ? '/>' : `>${tag.innerHTML ?? ''}</${tag.tagName}>`
}
/**
 *
 * @param {import('html-webpack-plugin').HtmlTagObject} tag
 */
function renderTag(tag) {
  return `${openTag(tag)}${closeTag(tag)}`
}

export class HtmlTemplateParameters {
  /**
   * @type {import('html-webpack-plugin').HtmlTagObject[]}
   */
  #headTags = []
  get headTags() {
    return this.#headTags.map(renderTag).join('')
  }

  /**
   * @type {import('html-webpack-plugin').HtmlTagObject[]}
   */
  #scriptTags = []
  get scriptTags() {
    return this.#scriptTags.map(renderTag).join('')
  }

  addStyleSheet(href) {
    this.#headTags.push({
      attributes: {
        href,
        rel: 'stylesheet',
      },
      tagName: 'link',
      voidTag: true,
    })

    return this
  }

  addScript(src, module) {
    this.#scriptTags.push({
      tagName: 'script',
      attributes: {
        src,
        ...(module ? {type: 'module'} : {defer: true}),
      },
    })
    return this
  }
}

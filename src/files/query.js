// @ts-check

/**
 *
 * @param {string} query
 * @returns {Promise<Object.<string, any>>}
 */
export const query = async query => {
  query = query.replace(/\n/gm, ' ').replace(/\s+/gm, ' ').trim()
  const res = await fetch('http://localhost:3000/query', {
    method: 'POST',
    body: query,
    headers: { 'Content-Type': 'text/plain' }
  })

  if (res.status === 200) {
    const json = await res.json()
    return json
  } else {
    let text = await res.text()
    showError(query, text)
    console.error(text)
    console.error(query)
    return []
  }
}

/**
 * Reads the image from a blob
 * @param {SubmitEvent} event
 * @param {Blob} image
 * @returns {Promise<String>} base64
 */
export const readImage = (event, image) => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = async event => {
      const res = event?.target?.result
      if (typeof res === 'string') resolve(res)
      else reject(res)
    }
    reader.readAsDataURL(image)
  })
}

/**
 * Will crop any image to 256x256 (max. 3mb)
 * @param {String} base64
 * @returns {Promise<String>}
 */
export const cropImage = async base64 => {
  let res = await fetch('http://localhost:3000/crop-image', {
    method: 'POST',
    body: base64,
    headers: { 'Content-Type': 'text/plain' }
  })
  const text = await res.text()
  if (res.status === 200) {
    return text
  } else {
    showError(null, text)
    console.error(text)
    return ''
  }
}

/**
 * Converts a SQL Row to an HTMLFormElement
 * @param {SubmitEvent} event
 * @returns {Object.<string, any>}
 */
export const formDataToObject = event => {
  const form = /** @type {HTMLFormElement} */ (event.target)
  const formData = new FormData(form)
  let data = {}
  for (const [name, value] of formData.entries()) {
    console.log(name, value, typeof value, typeof name)
    data = { ...data, [name]: value }
  }
  return data
}

/**
 *
 * @param {Array<Object.<string, any>>} rows
 * @returns {HTMLTableElement|void}
 */
export const toTable = rows => {
  if (Array.isArray(rows)) {
    if (rows.length === 0) rows.push({ empty: 'empty' })
    let table = '<table>'
    table += '<thead><tr>'
    table += Object.keys(rows[0])
      .map(key => `<th>${key}</th>`)
      .join('')
    table += '</tr></thead><tbody>'
    for (const row of rows) {
      table += '<tr>'
      table += Object.values(row)
        .map(value => {
          if (typeof value === 'string' && value.startsWith('data:image')) return `<td><img src="${value}"></td>`
          else return `<td>${value}</td>`
        })
        .join('')
      table += '</tr>'
    }
    table += '</tbody></table>'
    const el = /** @type {HTMLTableElement} */ (document.createElement('table'))
    el.innerHTML = table
    return el
  } else {
    showError(null, 'toTable() failed. Argument is not an Array.')
  }
}

/**
 *
 * @param {String|null} query
 * @param {String} error
 */
const showError = (query, error) => {
  // error wrapper
  let wrapper = document.getElementById('pg_proxy_wrapper')
  if (!wrapper) {
    wrapper = document.createElement('div')
    wrapper.id = 'pg_proxy_wrapper'
    wrapper.setAttribute(
      'style',
      `position: fixed;
      top: 0px;
      left: 0px;
      display: inline-block;`
    )
    document.body.append(wrapper)
  }

  // error element
  let errorEl = document.createElement('p')
  errorEl.classList.add('pg_proxy_error')
  errorEl.setAttribute(
    'style',
    `background: red;
    color: white;
    padding: 16px;
    margin: 16px;
    font-family: monospace;
    font-size: 16px;
    border-radius: 4px;`
  )
  errorEl.innerHTML = error
  if (query) errorEl.innerHTML += '<br/><br/>' + '<span><small>' + query + '</small></span>'
  wrapper.append(errorEl)
}

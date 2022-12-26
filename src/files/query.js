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
 *
 * @param {Array<Object.<string, any>>} rows
 * @returns {HTMLTableElement|void}
 */
export const toTable = rows => {
  if (Array.isArray(rows)) {
    let table = '<table>'
    table += '<thead><tr>'
    table += Object.keys(rows[0])
      .map(key => `<th>${key}</th>`)
      .join('')
    table += '</tr></thead><tbody>'
    for (const row of rows) {
      table += '<tr>'
      table += Object.values(row)
        .map(value => `<td>${value}</td>`)
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

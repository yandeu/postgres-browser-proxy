// @ts-check

// @ts-ignore
import { query, toTable, formDataToObject, readImage, cropImage } from 'http://localhost:3000/query.js'

// await query(/*sql*/ `DROP TABLE images;`)

await query(/*sql*/ `
  CREATE TABLE IF NOT EXISTS images (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    image text,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
  );
`)

const form = /** @type {HTMLFormElement} */ (document.querySelector('form'))
form.addEventListener('submit', async event => {
  event.preventDefault()

  const data = formDataToObject(event)
  const imageData = await readImage(event, data.image)
  const image = await cropImage(imageData)

  await query(/*sql*/ `
    INSERT INTO
      images (name, image)
    VALUES
      ('${data.name}','${image}')
  `)

  window.location.reload()
})

const images = await query(/*sql*/ `SELECT * FROM images ORDER BY created_at DESC;`)

document.body.append(toTable(images))

export {}

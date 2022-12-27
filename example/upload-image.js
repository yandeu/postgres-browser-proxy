// @ts-check

// @ts-ignore
import { query, toTable, formDataToObject, cropImage } from 'http://localhost:3000/query.js'

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
form.addEventListener('submit', event => {
  event.preventDefault()
  const data = formDataToObject(event)

  var reader = new FileReader()
  reader.onload = async event => {
    let image = event?.target?.result
    image = await cropImage(image)
    data.image = event?.target?.result
    await query(/*sql*/ `
      INSERT INTO
        images (name, image)
      VALUES
        ('${data.name}','${image}')
      `)
    // window.location.reload()
  }
  reader.readAsDataURL(data.image)
})

// await query(/*sql*/ `
//   INSERT INTO
//     users (username, age)
//   VALUES
//     ('Macy', 24),
//     ('Terry', NULL),
//     ('Evan', NULL);
// `)

const images = await query(/*sql*/ `SELECT * FROM images;`)
// console.log(users)

// const code = /** @type {HTMLElement} */ (document.querySelector('code'))
// code.innerHTML = JSON.stringify(users, null, 2)

document.body.append(toTable(images))

export {}

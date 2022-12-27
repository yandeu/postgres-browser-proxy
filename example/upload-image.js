// @ts-check

// @ts-ignore
import { query, toTable, formDataToObject, cropImage } from 'http://localhost:3000/query.js'

await query(/*sql*/ `
  CREATE TABLE IF NOT EXISTS images (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    image text DEFAULT "untitled",
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
    console.log('cropped', image)
    data.image = event?.target?.result
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

// const users = await query(/*sql*/ `SELECT * FROM users;`)
// console.log(users)

// const code = /** @type {HTMLElement} */ (document.querySelector('code'))
// code.innerHTML = JSON.stringify(users, null, 2)

// document.body.append(toTable(users))

export {}

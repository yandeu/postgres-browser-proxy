// @ts-check

// @ts-ignore
import { query, toTable } from 'http://localhost:3000/query.js'

await query(/*sql*/ `DROP TABLE users;`)

await query(/*sql*/ `
  CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    age INT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
  );
`)

await query(/*sql*/ `
  INSERT INTO
    users (username, age)
  VALUES
    ('Macy', 24),
    ('Terry', NULL),
    ('Evan', NULL);
`)

const users = await query(/*sql*/ `SELECT * FROM users;`)
console.log(users)

const code = /** @type {HTMLElement} */ (document.querySelector('code'))
code.innerHTML = JSON.stringify(users, null, 2)

document.body.append(toTable(users))

export {}

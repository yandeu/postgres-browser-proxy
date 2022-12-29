// @ts-check

// @ts-ignore
import { query, toTable, formDataToObject, readImage, cropImage } from 'http://localhost:3000/query.js'

await query(/*sql*/ `DROP TABLE places;`)

await query(/*sql*/ `
  CREATE TABLE IF NOT EXISTS places (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE,
    position geography,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
  );
`)

// create index for geography queries
await query(/*sql*/ `
  CREATE INDEX ON places using gist (position);
`)

await query(/*sql*/ `
    INSERT INTO
      places (name, position)
    VALUES
      ('New York','SRID=4326;POINT(-74.005974 40.712776)'),
      ('Paris','SRID=4326;POINT(48.856613 2.352222)'),
      ('Stockholm','SRID=4326;POINT(18.068581 59.329323)')
  `)

// https://www.latlong.net/
const ZURICH = {
  lat: 47.376888,
  long: 8.541694
}
const places = await query(/*sql*/ `
  SELECT * 
  FROM places 
  ORDER BY position <-> 'SRID=4326;POINT(${ZURICH.long} ${ZURICH.lat})' 
  limit 5;
`)

document.body.append(toTable(places))

export {}

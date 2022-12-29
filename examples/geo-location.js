// @ts-check

// @ts-ignore
import { query, toTable, calcCrow } from 'http://localhost:3000/query.js'

await query(/*sql*/ `DROP TABLE places;`)

await query(/*sql*/ `
  CREATE TABLE IF NOT EXISTS places (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE,
    position GEOGRAPHY,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
  );
`)

// create index for geography queries
await query(/*sql*/ `
  CREATE INDEX ON places USING gist (position);
`)

// POINT(long lat)
await query(/*sql*/ `
    INSERT INTO
      places (name, position)
    VALUES
      ('New York','SRID=4326;POINT(-74.005974 40.712776)'),
      ('Paris','SRID=4326;POINT(2.349014 48.864716)'),
      ('Stockholm','SRID=4326;POINT(18.068581 59.329323)')
  `)

const ZURICH = {
  long: 8.541694,
  lat: 47.376888
}
const places = await query(/*sql*/ `
  SELECT *
  FROM places
  ORDER BY position <-> 'SRID=4326;POINT(${ZURICH.long} ${ZURICH.lat})'
  limit 5;
`)
// alternative: ORDER BY ST_Distance(position::geography, ST_SetSRID(ST_Point(${ZURICH.long},${ZURICH.lat}), 4326)::geography)

document.body.append(toTable(places))

document.body.append(
  toTable(
    places.map(({ name, position: pos }) => {
      return { name, distance: Math.round(calcCrow(ZURICH.long, ZURICH.lat, pos.long, pos.lat)) + 'KM' }
    })
  )
)

export {}

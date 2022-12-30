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

// create index for geography queries (optional)
// await query(/*sql*/ `CREATE INDEX ON places USING gist (position);`)

// POINT(long lat)
await query(/*sql*/ `
  INSERT INTO
    places (name, position)
  VALUES
    ('New York','SRID=4326;POINT(-74.005974 40.712776)'),
    ('Paris','SRID=4326;POINT(2.349014 48.864716)'),
    ('Stockholm','SRID=4326;POINT(18.068581 59.329323)'),
    ('Vienna','SRID=4326;POINT(16.363449 48.210033)'),
    ('Geneva','SRID=4326;POINT(6.143158 46.204391)'),
    ('San Francisco','SRID=4326;POINT(-122.431297 37.773972)'),
    ('Tokio','SRID=4326;POINT(139.839478 35.652832)'),
    ('London','SRID=4326;POINT(-0.118092 51.509865)')
`)

const ZURICH = {
  long: 8.541694,
  lat: 47.376888
}

/**
 * Get all places within 20'000KM and sort by distance.
 * https://postgis.net/docs/ST_DWithin.html
 * https://postgis.net/docs/ST_Distance.html
 *
 * What does EPSG 4326 mean?
 * EPSG:4326 - WGS 84, latitude/longitude coordinate system based on the Earth's center of mass, used by the Global Positioning System among others.
 */
const places = await query(/*sql*/ `
  SELECT *
  FROM places
  WHERE ST_DWithin (
    position::geography,
    ST_SetSRID(ST_Point(${ZURICH.long},${ZURICH.lat}), 4326)::geography,
    20000000,
    true
  )
  ORDER BY ST_Distance(
    position::geography,
    ST_SetSRID(ST_Point(${ZURICH.long},${ZURICH.lat}), 4326)::geography
  )
`)

document.body.append(
  toTable(
    places.map(({ name: city, position: pos }) => {
      return { city, distance: Math.round(calcCrow(ZURICH.long, ZURICH.lat, pos.long, pos.lat)) + 'KM' }
    })
  )
)

document.body.append(toTable(places))

export {}

# Postgres Browser Proxy

Query your Postgres Database directly from the Browser.

**Video**:  
https://youtu.be/ohr9gBPC3cE

**Download the latest binaries**:  
https://github.com/yandeu/postgres-browser-proxy/releases

**Add more types to it**:  
https://docs.rs/postgres/latest/postgres/types/trait.FromSql.html#types

## How

The proxy is by default running on port 3000 and is connecting to Postgres on localhost:5432 with user "postgres" and password "mysecretpassword".

```bash
# start postgres (using docker)
docker run --name some-postgres -p 5432:5432 -e POSTGRES_PASSWORD=mysecretpassword -d postgres:15-alpine
```

### Windows

```pwsh
 .\postgres-browser-proxy.exe
```

### Linux/MacOS

```bash
chmod +x postgres-browser-proxy
./postgres-browser-proxy
```

### CLI Options

```pwsh
Usage: postgres-browser-proxy.exe [OPTIONS]

Options:
      --host <HOST>          [default: localhost]
      --user <USER>          [default: postgres]
      --password <PASSWORD>  [default: mysecretpassword]
      --port <PORT>          [default: 3000]
      --pg-port <PG_PORT>    [default: 5432]
  -h, --help                 Print help information
  -V, --version              Print version information
```

## Example

It looks like this in you client-side javascript.  
_More examples are in the [examples directory](./examples)._

```js
// @ts-check

// @ts-ignore
import { query } from 'http://localhost:3000/query.js'

// await query(/*sql*/ `DROP TABLE users;`)

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
```

## License

Licensed under GPLv3
Copyright (c) 2022, [Yannick Deubel](https://github.com/yandeu)  
Please have a look at the [LICENSE](https://github.com/yandeu/postgres-browser-proxy/blob/main/LICENSE) for more details.

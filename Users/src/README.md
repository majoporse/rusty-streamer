```bash
diesel migration run 
diesel database setup 
diesel migration generate --diff-schema init 
diesel database drop
```
set DATABASE_URL=postgres://postgres:postgres@localhost:5432/movies

$env:DATABASE_URL = "postgres://postgres:postgres@localhost:5432/movies"

export DATABASE_URL=postgres://postgres:postgres@localhost:5432/movies

$env:LIB = "C:\Program Files\PostgreSQL\18\lib"
$env:INCLUDE = "C:\Program Files\PostgreSQL\18\include"
$env:PATH = "C:\Program Files\PostgreSQL\18\bin;" + $env:PATH

# Docker run and build
1. `cd te-take-of && docker build --network=host -t backend:0.1.0 .`
2. `docker run --network=host --expose 8000 -p 8000:8000/tcp --env-file .env backend:0.1.0`

## Dependencies: you need to have postgres running with exposed port 8015 example:
 `docker run --name local-postgres -p 127.0.0.1:8015:5432 -e POSTGRES_PASSWORD=password -d postgres`

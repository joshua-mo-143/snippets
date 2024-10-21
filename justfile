up:
    concurrently "npm --prefix frontend run dev" "shuttle run"

shuttle-db:
    docker exec -it shuttle_rustytips_shared_postgres psql -p 5432 -h localhost -U postgres -d rustytips

build:
    npm --prefix frontend run build && mv frontend/dist dist && shuttle deploy --ad

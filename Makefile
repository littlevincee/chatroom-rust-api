# cargo run a package
cargo run -p auth-service

# dev
systemfd --no-pid -s http::5001 -- cargo watch -x run

# diesel migration
diesel migration generate <name>

# diesel setup
diesel setup

#diesel apply new migration
diesel migration run

#diesel migration rollback
diesel migration redo

# docker build
docker build -t name .

# docker compose up
docker-compose -f docker-compose.override.yml up

# wait-for-it.sh
# change it to LF endline
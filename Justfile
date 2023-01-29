setup:
    sqlx database create && sqlx migrate run --source crates/axios_genie_db/migrations

run:
    cargo run

# export:
#     export $(cat .env | xargs)

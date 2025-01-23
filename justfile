set windows-powershell := true
set dotenv-load
set dotenv-required

# DATABASE_URL:="sqlite://{{source_dir()}}/sqlite.db"

default:
    just --list

setup-tools:
    @cargo install sqlx-cli --no-default-features --features=sqlite
    @cargo install bacon

setup-db:
    @sqlx database create
    @just migrate run

dev-server:
    @bacon --headless

dev-client:
    @cd frontend
    @npm run dev

migration-new name:
    @sqlx migrate add -t -r {{name}}

migrate command:
    @sqlx migrate {{command}}

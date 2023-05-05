# Rust Github CI/CD

Project to experiment with Github Actions.

## Start

Copy the `.env.example`:

```shell
cp .env.example .env
```

Start docker:

```shell
docker compose up -d
```

Go to: [localhost:2022](http://localhost:3001/_health)

## FAQ

<summary>
	<details><strong>(⬇️ Not needed with Docker ⬇️)<br>
		Install SQLx CLI to check the `query!` macro without
		needing a DB (ex: in a CI environment):</strong>
	</details>

```shell
cargo install sqlx-cli
```

</summary>

<summary>
	<details><strong>To enter the `psql` console of the Postgres:</strong></details>

```shell
docker compose exec -e PGPASSWORD=CHANGEME -it db psql -U city-api city-db
```

</summary>

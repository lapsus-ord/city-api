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

<details>
	<summary>
		<strong>
			Install SQLx CLI to check the <code>query!</code> macro without
			needing a DB (ex: in a CI environment)
		</strong>
	</summary>

> ️ℹ️ Not needed with Docker

```shell
cargo install sqlx-cli
```

Source: [Enable building in "offline mode" with `query!()`
](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli#enable-building-in-offline-mode-with-query)

</details>

<br>

<details>
	<summary><strong>To enter the <code>psql</code> console of the Postgres</strong></summary>

```shell
docker compose exec -e PGPASSWORD=CHANGEME -it db psql -U city-api city-db
```

</details>

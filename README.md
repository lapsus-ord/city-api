# Rust Github CI/CD

Project to experiment with Github Actions.

## Start

Copy the `.env.example`:

```shell
cp .env.example .env
```

Start DB & api:

```shell
docker compose up -d
```

Go to: [localhost:2022](http://localhost:2022/_health)

<details>
	<summary>
		If you want to use your local version, change in the [`compose.yaml`](./compose.yaml):
	</summary>

```yaml
services:
  api:
    image: ghcr.io/lapsus-ord/city-api:latest
  # ...
```

by:

```yaml
services:
  api:
    build:
      context: .
      target: production
    # ...
```

</details>

## Launch on K3D

- First, install [k3d](https://k3d.io/#installation) (or minikube)
- Then, create a cluster: `k3d cluster create city-api-cluster`
- Deploy using helm: `helm install city-api ./helm`
- Open the service: `kubectl port-forward deployment/city-api-deployment 2022`

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

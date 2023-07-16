# Rust Github CI/CD

Project to experiment with Github Actions.

## Launch the app on Docker

### Requirements

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

### Instructions

- Copy the `.env.example`:

```shell
cp .env.example .env
```

- Start DB & api:

```shell
docker compose up -d
```

- Access the application here: [localhost:2022](http://localhost:2022/_health)

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

## Launch the app on K3D

### Requirements

- [helm](https://helm.sh/docs/intro/install/)
- [kubectl](https://kubernetes.io/docs/tasks/tools/install-kubectl/)

### Instructions

- Install [k3d](https://k3d.io/#installation) (alternatively, you can use [minikube](https://minikube.sigs.k8s.io/docs/start/)):

```shell
wget -q -O - https://raw.githubusercontent.com/k3d-io/k3d/main/install.sh | bash
```

- Create a cluster: 

```shell
k3d cluster create city-api-cluster
```

- Deploy the application using helm: 

```
helm install city-api ./helm
```

- Port forward the deployment to your local machine:

```shell
kubectl port-forward deployment/city-api-deployment 2022
```

- Access the application here: [localhost:2022](http://localhost:2022/_health)

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

# Keploy: Record & Replay HTTP Tests (Local)

Goal: deterministically capture and replay external HTTP calls while we run the app locally. This helps validate integrations without hitting live services.

## Options

1) Keploy CLI installed on host

```
./scripts/keploy-record.sh   # run app under keploy in record mode
./scripts/keploy-replay.sh   # replay recorded testcases
```

2) Docker compose (no host installation)

```
docker compose -f docker-compose.keploy.yml up --build
```

This will start keploy and build + run the app under test. By default, `KEPLOY_MODE=record`.

## Files

- `keploy.yaml` – config; testcases saved under `keploy/testcases`, mocks under `keploy/mocks`
- `docker-compose.keploy.yml` – keploy and app services (host network)
- `Dockerfile.keploy` – production build for the app service
- `scripts/keploy-record.sh`, `scripts/keploy-replay.sh` – host-run wrappers for CLI

## Notes

- We treat GitHub as VCS only; Keploy tests run locally on this server.
- Keep large captured datasets out of `main`; store them on feature branches until validated.


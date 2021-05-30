# Tik-Giphy

Web app for browsing giphy trending gifs.

Backend portion of this is a homework task for UAB Nikulipe

## Running this app

### Release/Prod

1. Create `.env` file, you can use `example.env` for this:

```bash
cp example.env .env
```

2. Run docker compose with prod compose files:

```bash
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up
```

### Development

1. Create `.env` file, you can use `example.env` for this:

```bash
cp example.env .env
```

2. Run docker compose:

```bash
docker compose up -d
```

3. Go to desired folder and run

```
cargo cmd dev
```

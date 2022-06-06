dev-start:
	docker compose up -d postgres
	docker compose up strava_scraper

dev-reset:
	docker compose down
	docker system prune -af

export:
	@pg_dump --table=user_stats --data-only --column-inserts stats > data.sql

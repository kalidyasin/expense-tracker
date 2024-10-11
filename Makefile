run:
	cargo run

test:
	cargo test

build:
	cargo build --release

clean:
	cargo clean

start-docker:
	@if ! systemctl is-active --quiet docker; then \
		echo "Starting Docker..."; \
		service docker start; \
		echo "Docker started."; \
	else \
		echo "Docker is already running."; \
	fi

stop-db:
	@if [ "$(shell docker ps -q -f name=expense-tracker-db)" ]; then \
		echo "Stopping container 'expense-tracker-db'..."; \
		docker stop expense-tracker-db; \
		echo "Container 'expense-tracker-db' stopped."; \
	else \
		echo "Container 'expense-tracker-db' is not running."; \
	fi

clean-db:
	@if [ "$(shell docker ps -aq -f name=expense-tracker-db)" ]; then \
		echo "Removing container 'expense-tracker-db'..."; \
		docker rm expense-tracker-db; \
		echo "Container 'expense-tracker-db' removed."; \
	else \
		echo "Container 'expense-tracker-db' does not exist."; \
	fi

run-db:
	@echo "Starting container 'expense-tracker-db'..."; \
	docker run --name expense-tracker-db -p 5432:5432 -e POSTGRES_PASSWORD=postgres -v ./db-data:/var/lib/postgresql/data -d postgres:alpine && \
	echo "Container 'expense-tracker-db' started."

stop-docker:
	@if systemctl is-active --quiet docker; then \
		echo "Stopping Docker..."; \
		service docker stop; \
		echo "Docker stopped."; \
	else \
		echo "Docker is not running."; \
	fi

start-db: start-docker stop-db clean-db run-db

stop: stop-db clean-db


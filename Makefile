.DEFAULT_GOAL := help

.PHONY: help
help: ## Show this help
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[1m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

.PHONY: serve
serve: ## Serve (web)
	@dx serve --platform web 

.PHONY: tailwind
tailwind: ## Run tailwind
	@npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch

build:
	@echo "-----------------------------------"
	@echo "Building lambda"
	sam build --beta-features

start:
	@echo "-----------------------------------"
	@echo "start lambda locally"
	sam local start-api

run: build start

invoke: 
	@echo "-----------------------------------"
	@echo "invoke lambda test root"
	@echo "-----------------------------------"
	sam local invoke WelcomeFunction --event ./events/test-root.json
	@echo ""
	@echo "-----------------------------------"
	@echo "invoke lambda test welcome"
	@echo "-----------------------------------"
	sam local invoke WelcomeFunction --event ./events/test-welcome.json

.PHONY: build start invoke run

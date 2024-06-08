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
	@echo "invoke lambda locally"
	curl http://localhost:3000/v1/
	curl http://localhost:3000/v1/welcome/

.PHONY: build start invoke run

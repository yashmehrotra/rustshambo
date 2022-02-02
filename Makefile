API_IMG := yashmehrotra/rustshambo-api:latest
WEB_IMG := yashmehrotra/rustshambo-web:latest

docker-build-api:
	docker build -t ${API_IMG} .

docker-push-api:
	docker push ${API_IMG}

docker-build-web:
	cd frontend && docker build -t ${WEB_IMG} .

docker-push-web:
	docker push ${WEB_IMG}

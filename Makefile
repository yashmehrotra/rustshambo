IMG := yashmehrotra/rustshambo:latest

docker-build:
	docker build -t ${IMG} .

docker-push:
	docker push ${IMG}

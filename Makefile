build:
	docker build . -t itcp

run:
	docker run -it -v $(shell pwd):/usr/src/app itcp 
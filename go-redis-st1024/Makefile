all:
	time go run .
	
linux:	
	redis-server
	
docker:
	# start a redis instance
	docker run --rm --name redis5 -d redis -p 127.0.0.1:6379:6379
	docker ps -a
	docker exec -it redis2 sh
	uname -a
	redis-cli
	ping
	set a 1
	get a
	exit
	exit
	# Access Redis from Another Docker Container:
	docker run -it --rm --name redis3 --link redis2:redis -d redis
	docker exec -it redis3 sh
	redis-cli -h redis
	get a
	exit
	exit
	docker inspect bridge


	# start a redis instance:
	mkdir -p /r/redis
	docker run --name redis1 -d redis redis-server --appendonly yes -v /r/redis:/data
	docker network ls
	docker inspect host
	docker inspect none

	# connecting via redis-cli
	docker run -it --network none --rm redis redis-cli -h redis1

init:
	go get github.com/go-redis/redis/v8
	go get
	git log --graph --oneline --all
	
small:
	# make small binary:
	go build -ldflags "-s -w" .
	ls -lh

run:
	go run .

cost:	
	go build -gcflags=-m=2 . 2>&1 | code -
	
test:
	go test

bench:	
	go test -benchtime=400000000x -benchmem -run=^$ -bench .

push:
	git push

sameAsRemote:
	git fetch origin
	git reset --hard origin/main

ck:
	# git fsck
	# https://github.com/dominikh/go-tools
	# https://staticcheck.io/docs/
	staticcheck ./...

	# gc:
	# git gc

lines:
	# git ls-files | grep "\(.html\|.css\|.js\|.go\)$" | xargs wc -l
	git ls-files | xargs cat | wc -l

race:
	# The race detector is fully integrated with the Go toolchain. 
	# build your code with the race detector enabled, 
	# just add the -race flag to the command line:
	go build -race .  	

wasm:
	# cp "$(go env GOROOT)/misc/wasm/wasm_exec.js" .
	GOOS=js GOARCH=wasm go build -o view/main.wasm	
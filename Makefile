# use make build tag=vN to build a specific tag
# otherwise, the lexicographically highest tag will be found on
.PHONY: build push help
.DEFAULT_GOAL := help

REGISTRY := https://registry.hub.docker.com
REPO := numberoverzero
IMAGE := rook-action

ifdef tag
VERSION := $(tag)
else
# v1 -> v2, v10 -> v11
OLD_VERSION := $(shell wget -q "${REGISTRY}/v2/repositories/${REPO}/${IMAGE}/tags" -O - | jq -r .results[].name | sort -fV | tail -n 1)
VERSION := $(shell echo ${OLD_VERSION} | tr -cd [a-z,A-Z])$(shell echo $$(($(shell echo ${OLD_VERSION} | tr -cd [0-9])+1)))
endif

FULLIMAGE := ${REPO}/${IMAGE}:${VERSION}

build:
	@echo FULLIMAGE=${FULLIMAGE}
	@docker build -t ${FULLIMAGE} .
	@docker image ls ${FULLIMAGE}

push:
	@echo FULLIMAGE=${FULLIMAGE}
	@docker push ${FULLIMAGE}

help:
	@echo 'make build will execute the following:'
	@echo '    docker build -t ${FULLIMAGE} .'
	@echo 'make push will execute the following:'
	@echo '    docker push ${FULLIMAGE}'

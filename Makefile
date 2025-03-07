.PHONY: setup dev-env test build clean help

# 기본 목표
.DEFAULT_GOAL := help

# 셸 설정
SHELL := /bin/bash

# 버전 정보
VERSION := 0.1.0

# 디렉토리
SRC_DIR := .
CORE_DIR := ./core
ANALYTICS_DIR := ./analytics
API_DIR := ./api
PLUGINS_DIR := ./plugins
EXAMPLES_DIR := ./examples
BUILD_DIR := ./build
DIST_DIR := ./dist

# 명령어
CARGO := cargo
GO := go
DOCKER := docker
DOCKER_COMPOSE := docker-compose

# 색상 설정
BLUE := \033[34m
GREEN := \033[32m
YELLOW := \033[33m
RED := \033[31m
NC := \033[0m # No Color

# 도움말
help: ## 사용 가능한 명령어 목록 표시
	@echo -e "$(BLUE)CryptoLytica$(NC) v$(VERSION) - 암호화폐 데이터 분석 플랫폼"
	@echo -e "사용 방법: make [target]"
	@echo ""
	@echo -e "$(YELLOW)사용 가능한 명령어:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-20s$(NC) %s\n", $$1, $$2}'

# 초기 설정
setup: ## 개발 환경 초기 설정
	@echo -e "$(BLUE)CryptoLytica$(NC) 개발 환경을 설정합니다..."
	@mkdir -p $(CORE_DIR) $(ANALYTICS_DIR) $(API_DIR) $(PLUGINS_DIR) $(EXAMPLES_DIR)
	@echo -e "$(GREEN)✓$(NC) 프로젝트 디렉토리 구조 생성 완료"
	@echo -e "$(YELLOW)도움말:$(NC) 프로젝트를 시작하려면 'make dev-env' 명령어를 실행하세요."

# 개발 환경
dev-env: ## 개발 환경 실행
	@echo -e "$(BLUE)CryptoLytica$(NC) 개발 환경을 시작합니다..."
	@echo -e "$(GREEN)✓$(NC) 개발 환경 설정 완료"
	@echo -e "$(YELLOW)안내:$(NC) 이제 개발을 시작할 수 있습니다."

# 테스트
test: ## 테스트 실행
	@echo -e "$(BLUE)CryptoLytica$(NC) 테스트를 실행합니다..."
	@echo -e "$(YELLOW)안내:$(NC) 테스트 환경이 아직 구성되지 않았습니다."

# 빌드
build: ## 프로젝트 빌드
	@echo -e "$(BLUE)CryptoLytica$(NC) 빌드를 시작합니다..."
	@mkdir -p $(BUILD_DIR)
	@echo -e "$(YELLOW)안내:$(NC) 빌드 환경이 아직 구성되지 않았습니다."

# 정리
clean: ## 생성된 파일 및 디렉토리 정리
	@echo -e "$(BLUE)CryptoLytica$(NC) 정리를 시작합니다..."
	@rm -rf $(BUILD_DIR) $(DIST_DIR)
	@echo -e "$(GREEN)✓$(NC) 정리 완료"
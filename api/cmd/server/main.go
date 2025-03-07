package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/kimjooyoon/cryptolytica/api/internal/config"
	"github.com/kimjooyoon/cryptolytica/api/internal/handlers"
	"github.com/kimjooyoon/cryptolytica/api/internal/middleware"
	"go.uber.org/zap"
)

func main() {
	// 로거 초기화
	logger, err := zap.NewProduction()
	if err != nil {
		log.Fatalf("로거 초기화 실패: %v", err)
	}
	defer logger.Sync()

	// 설정 로드
	cfg, err := config.Load()
	if err != nil {
		logger.Fatal("설정 로드 실패", zap.Error(err))
	}

	// Gin 라우터 설정
	router := setupRouter(cfg, logger)

	// HTTP 서버 설정
	server := &http.Server{
		Addr:         fmt.Sprintf(":%d", cfg.Server.Port),
		Handler:      router,
		ReadTimeout:  time.Duration(cfg.Server.ReadTimeoutSeconds) * time.Second,
		WriteTimeout: time.Duration(cfg.Server.WriteTimeoutSeconds) * time.Second,
		IdleTimeout:  time.Duration(cfg.Server.IdleTimeoutSeconds) * time.Second,
	}

	// 서버를 고루틴으로 시작
	go func() {
		logger.Info("HTTP 서버 시작", zap.String("addr", server.Addr))
		if err := server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			logger.Fatal("서버 시작 실패", zap.Error(err))
		}
	}()

	// 종료 신호 대기
	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit
	logger.Info("서버 종료 시작...")

	// 정상 종료를 위한 컨텍스트 설정
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	// 서버 정상 종료
	if err := server.Shutdown(ctx); err != nil {
		logger.Fatal("서버 강제 종료", zap.Error(err))
	}

	logger.Info("서버 정상 종료")
}

// 라우터 설정
func setupRouter(cfg *config.Config, logger *zap.Logger) *gin.Engine {
	// 릴리스 모드 설정
	if cfg.Server.Mode == "release" {
		gin.SetMode(gin.ReleaseMode)
	}

	router := gin.New()

	// 미들웨어 설정
	router.Use(middleware.Logger(logger))
	router.Use(middleware.Recovery(logger))
	router.Use(middleware.CORS())
	router.Use(middleware.RateLimiter(cfg.RateLimit.RequestsPerSecond, cfg.RateLimit.BurstSize))

	// 상태 확인 엔드포인트
	router.GET("/health", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"status":  "up",
			"version": "0.1.0",
			"time":    time.Now().Format(time.RFC3339),
		})
	})

	// API 라우트 그룹
	api := router.Group("/api")
	{
		// v1 API 그룹
		v1 := api.Group("/v1")
		{
			// 거래소 라우트
			exchanges := v1.Group("/exchanges")
			{
				exchanges.GET("", handlers.GetExchanges)
				exchanges.GET("/:exchange", handlers.GetExchange)
			}

			// 시장 데이터 라우트
			markets := v1.Group("/markets")
			{
				markets.GET("", handlers.GetMarkets)
				markets.GET("/:symbol", handlers.GetMarket)
				markets.GET("/:symbol/ticker", handlers.GetTicker)
				markets.GET("/:symbol/orderbook", handlers.GetOrderBook)
				markets.GET("/:symbol/trades", handlers.GetTrades)
				markets.GET("/:symbol/candles", handlers.GetCandles)
			}

			// 분석 라우트
			analytics := v1.Group("/analytics")
			{
				analytics.GET("/summary", handlers.GetMarketSummary)
				analytics.GET("/volatility", handlers.GetVolatility)
				analytics.GET("/correlation", handlers.GetCorrelation)
				analytics.GET("/portfolio", handlers.AnalyzePortfolio)
			}
		}
	}

	// WebSocket 엔드포인트
	router.GET("/ws", handlers.WebSocketHandler)

	return router
}
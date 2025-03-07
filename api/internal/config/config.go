package config

import (
	"fmt"
	"strings"

	"github.com/spf13/viper"
)

// Config는 애플리케이션 설정을 담는 구조체입니다.
type Config struct {
	Server     ServerConfig     `mapstructure:"server"`
	Database   DatabaseConfig   `mapstructure:"database"`
	RateLimit  RateLimitConfig  `mapstructure:"rate_limit"`
	Logging    LoggingConfig    `mapstructure:"logging"`
	Core       CoreConfig       `mapstructure:"core"`
	Exchanges  ExchangesConfig  `mapstructure:"exchanges"`
	Auth       AuthConfig       `mapstructure:"auth"`
	Monitoring MonitoringConfig `mapstructure:"monitoring"`
}

// ServerConfig는 서버 관련 설정입니다.
type ServerConfig struct {
	Port                int    `mapstructure:"port"`
	Mode                string `mapstructure:"mode"`
	ReadTimeoutSeconds  int    `mapstructure:"read_timeout_seconds"`
	WriteTimeoutSeconds int    `mapstructure:"write_timeout_seconds"`
	IdleTimeoutSeconds  int    `mapstructure:"idle_timeout_seconds"`
	TrustedProxies      []string `mapstructure:"trusted_proxies"`
}

// DatabaseConfig는 데이터베이스 관련 설정입니다.
type DatabaseConfig struct {
	Type          string `mapstructure:"type"`
	Host          string `mapstructure:"host"`
	Port          int    `mapstructure:"port"`
	Username      string `mapstructure:"username"`
	Password      string `mapstructure:"password"`
	Database      string `mapstructure:"database"`
	MaxConnections int   `mapstructure:"max_connections"`
	SSLMode       string `mapstructure:"ssl_mode"`
}

// RateLimitConfig는 요청 속도 제한 관련 설정입니다.
type RateLimitConfig struct {
	Enabled           bool  `mapstructure:"enabled"`
	RequestsPerSecond int   `mapstructure:"requests_per_second"`
	BurstSize         int   `mapstructure:"burst_size"`
	ExpirySeconds     int   `mapstructure:"expiry_seconds"`
}

// LoggingConfig는 로깅 관련 설정입니다.
type LoggingConfig struct {
	Level      string `mapstructure:"level"`
	Format     string `mapstructure:"format"`
	OutputPath string `mapstructure:"output_path"`
}

// CoreConfig는 코어 라이브러리 관련 설정입니다.
type CoreConfig struct {
	DataDir          string `mapstructure:"data_dir"`
	CacheSize        int    `mapstructure:"cache_size"`
	MaxConcurrency   int    `mapstructure:"max_concurrency"`
	DefaultTimeoutMs int    `mapstructure:"default_timeout_ms"`
}

// ExchangesConfig는 암호화폐 거래소 관련 설정입니다.
type ExchangesConfig struct {
	Enabled  []string               `mapstructure:"enabled"`
	Settings map[string]interface{} `mapstructure:"settings"`
}

// AuthConfig는 인증 관련 설정입니다.
type AuthConfig struct {
	JWTSecret        string `mapstructure:"jwt_secret"`
	JWTExpiryMinutes int    `mapstructure:"jwt_expiry_minutes"`
	APIKeyEnabled    bool   `mapstructure:"api_key_enabled"`
}

// MonitoringConfig는 모니터링 관련 설정입니다.
type MonitoringConfig struct {
	PrometheusEnabled bool   `mapstructure:"prometheus_enabled"`
	MetricsPath       string `mapstructure:"metrics_path"`
	TracingEnabled    bool   `mapstructure:"tracing_enabled"`
}

// Load는 설정 파일을 로드하고 Config 구조체를 반환합니다.
func Load() (*Config, error) {
	// Viper 설정
	v := viper.New()
	v.SetConfigName("config")
	v.SetConfigType("yaml")
	v.AddConfigPath(".")
	v.AddConfigPath("./config")
	v.AddConfigPath("../config")
	v.AddConfigPath("/etc/cryptolytica")
	v.AddConfigPath("$HOME/.cryptolytica")

	// 환경 변수 대체
	v.SetEnvPrefix("CRYPTOLYTICA")
	v.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))
	v.AutomaticEnv()

	// 기본값 설정
	setDefaults(v)

	// 설정 파일 읽기
	if err := v.ReadInConfig(); err != nil {
		// 설정 파일을 찾을 수 없는 경우 경고만 출력하고 기본값 사용
		if _, ok := err.(viper.ConfigFileNotFoundError); !ok {
			return nil, fmt.Errorf("설정 파일 읽기 오류: %w", err)
		}
	}

	// 설정값을 Config 구조체로 언마샬링
	var config Config
	if err := v.Unmarshal(&config); err != nil {
		return nil, fmt.Errorf("설정 언마샬링 오류: %w", err)
	}

	return &config, nil
}

// setDefaults는 기본 설정값을 설정합니다.
func setDefaults(v *viper.Viper) {
	// 서버 설정 기본값
	v.SetDefault("server.port", 8080)
	v.SetDefault("server.mode", "release")
	v.SetDefault("server.read_timeout_seconds", 30)
	v.SetDefault("server.write_timeout_seconds", 30)
	v.SetDefault("server.idle_timeout_seconds", 60)
	v.SetDefault("server.trusted_proxies", []string{})

	// 데이터베이스 설정 기본값
	v.SetDefault("database.type", "memory")
	v.SetDefault("database.host", "localhost")
	v.SetDefault("database.port", 5432)
	v.SetDefault("database.username", "cryptolytica")
	v.SetDefault("database.password", "")
	v.SetDefault("database.database", "cryptolytica")
	v.SetDefault("database.max_connections", 50)
	v.SetDefault("database.ssl_mode", "disable")

	// 속도 제한 설정 기본값
	v.SetDefault("rate_limit.enabled", true)
	v.SetDefault("rate_limit.requests_per_second", 100)
	v.SetDefault("rate_limit.burst_size", 200)
	v.SetDefault("rate_limit.expiry_seconds", 60)

	// 로깅 설정 기본값
	v.SetDefault("logging.level", "info")
	v.SetDefault("logging.format", "json")
	v.SetDefault("logging.output_path", "stdout")

	// 코어 설정 기본값
	v.SetDefault("core.data_dir", "./data")
	v.SetDefault("core.cache_size", 1000)
	v.SetDefault("core.max_concurrency", 10)
	v.SetDefault("core.default_timeout_ms", 30000)

	// 거래소 설정 기본값
	v.SetDefault("exchanges.enabled", []string{"binance", "coinbase"})
	v.SetDefault("exchanges.settings", map[string]interface{}{})

	// 인증 설정 기본값
	v.SetDefault("auth.jwt_secret", "change-me-in-production")
	v.SetDefault("auth.jwt_expiry_minutes", 60)
	v.SetDefault("auth.api_key_enabled", true)

	// 모니터링 설정 기본값
	v.SetDefault("monitoring.prometheus_enabled", true)
	v.SetDefault("monitoring.metrics_path", "/metrics")
	v.SetDefault("monitoring.tracing_enabled", false)
}
package observability

import (
	"context"
	"net/url"

	"github.com/sygmaprotocol/sygma-core/observability/metrics"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/exporters/otlp/otlpmetric/otlpmetrichttp"
	"go.opentelemetry.io/otel/metric"
	api "go.opentelemetry.io/otel/metric"
	sdkmetric "go.opentelemetry.io/otel/sdk/metric"
	"go.opentelemetry.io/otel/sdk/metric/aggregation"
	sdkresource "go.opentelemetry.io/otel/sdk/resource"
	semconv "go.opentelemetry.io/otel/semconv/v1.17.0"
)

func InitMetricProvider(ctx context.Context, agentURL string, opts ...sdkmetric.Option) (*sdkmetric.MeterProvider, error) {
	collectorURL, err := url.Parse(agentURL)
	if err != nil {
		return nil, err
	}

	metricOptions := []otlpmetrichttp.Option{
		otlpmetrichttp.WithURLPath(collectorURL.Path),
		otlpmetrichttp.WithEndpoint(collectorURL.Host),
	}
	if collectorURL.Scheme == "http" {
		metricOptions = append(metricOptions, otlpmetrichttp.WithInsecure())
	}

	metricHTTPExporter, err := otlpmetrichttp.New(ctx, metricOptions...)
	if err != nil {
		return nil, err
	}

	httpMetricReader := sdkmetric.NewPeriodicReader(metricHTTPExporter)

	opts = append(opts, sdkmetric.WithReader(httpMetricReader))
	opts = append(opts, sdkmetric.WithResource(initResource()))
	opts = append(opts, sdkmetric.WithView(initSecondView()))
	opts = append(opts, sdkmetric.WithView(initGasView()))
	meterProvider := sdkmetric.NewMeterProvider(
		opts...,
	)
	return meterProvider, nil
}

func initResource() *sdkresource.Resource {
	res, _ := sdkresource.New(context.Background(),
		sdkresource.WithProcess(),
		sdkresource.WithTelemetrySDK(),
		sdkresource.WithHost(),
		sdkresource.WithAttributes(
			semconv.ServiceName("relayer"),
		),
	)
	return res
}

func initSecondView() sdkmetric.View {
	return sdkmetric.NewView(
		sdkmetric.Instrument{
			Unit: "s",
		},
		sdkmetric.Stream{
			Aggregation: aggregation.ExplicitBucketHistogram{
				Boundaries: []float64{
					0.000001, // 1 µs
					0.00001,  // 10 µs
					0.0001,   // 100 µs
					0.001,    // 1 ms
					0.005,    // 5 ms
					0.01,     // 10 ms
					0.05,     // 50 ms
					0.1,      // 100 ms
					0.5,      // 500 ms
					1.0,      // 1 s
					5.0,      // 5 s
					10.0,     // 10 s
					100.0,    // 100s
					1000.0,   // 1000s
					10000.0,  // 10000s
				},
				NoMinMax: false,
			},
		},
	)
}

func initGasView() sdkmetric.View {
	return sdkmetric.NewView(
		sdkmetric.Instrument{
			Unit: "gas",
		},
		sdkmetric.Stream{
			Aggregation: aggregation.ExplicitBucketHistogram{
				Boundaries: []float64{
					10000,
					20000,
					50000,
					100000,
					500000,
					1000000,
					5000000,
					10000000,
					15000000,
					30000000,
				},
				NoMinMax: false,
			},
		},
	)
}

type RelayerMetrics struct {
	*metrics.SystemMetrics
	*metrics.MessageMetrics
	*metrics.ChainMetrics

	Opts api.MeasurementOption
}

// NewRelayerMetrics initializes OpenTelemetry metrics
func NewRelayerMetrics(ctx context.Context, meter metric.Meter, attributes ...attribute.KeyValue) (*RelayerMetrics, error) {
	opts := api.WithAttributes(attributes...)

	systemMetrics, err := metrics.NewSystemMetrics(ctx, meter, opts)
	if err != nil {
		return nil, err
	}

	messageMetrics, err := metrics.NewMessageMetrics(ctx, meter, opts)
	if err != nil {
		return nil, err
	}

	chainMetrics, err := metrics.NewChainMetrics(ctx, meter, opts)
	if err != nil {
		return nil, err
	}

	return &RelayerMetrics{
		SystemMetrics:  systemMetrics,
		ChainMetrics:   chainMetrics,
		MessageMetrics: messageMetrics,
		Opts:           opts,
	}, err
}

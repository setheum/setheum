package metrics

import (
	"context"
	"time"

	"go.opentelemetry.io/otel/metric"
	api "go.opentelemetry.io/otel/metric"
)

type HostMetrics struct {
	startTimeGauge api.Int64ObservableGauge
}

// NewHostMetrics initializes metrics related to the relayer host
func NewHostMetrics(ctx context.Context, meter metric.Meter, opts metric.MeasurementOption) (*HostMetrics, error) {
	startTime := time.Now().Unix()
	startTimeGauge, err := meter.Int64ObservableGauge(
		"relayer.StartTimeSeconds",
		api.WithDescription("Start time of the relayer"),
		api.WithInt64Callback(func(ctx context.Context, result api.Int64Observer) error {
			result.Observe(startTime, opts)
			return nil
		}),
	)
	if err != nil {
		return nil, err
	}

	return &HostMetrics{
		startTimeGauge: startTimeGauge,
	}, nil
}

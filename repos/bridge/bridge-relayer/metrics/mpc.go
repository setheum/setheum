package metrics

import (
	"context"

	"github.com/libp2p/go-libp2p/core/peer"
	"go.opentelemetry.io/otel/metric"
	api "go.opentelemetry.io/otel/metric"
)

type MpcMetrics struct {
	totalRelayersGauge     api.Int64ObservableGauge
	availableRelayersGauge api.Int64ObservableGauge
	totalRelayerCount      *int64
	availableRelayerCount  *int64
}

// NewMpcMetrics initializes metrics related to the MPC set
func NewMpcMetrics(ctx context.Context, meter metric.Meter, opts metric.MeasurementOption) (*MpcMetrics, error) {
	totalRelayerCount := new(int64)
	availableRelayerCount := new(int64)
	totalRelayersGauge, err := meter.Int64ObservableGauge(
		"relayer.TotalRelayers",
		api.WithInt64Callback(func(context context.Context, result api.Int64Observer) error {
			result.Observe(*totalRelayerCount, opts)
			return nil
		}),
		api.WithDescription("Total number of relayers currently in the subset"),
	)
	if err != nil {
		return nil, err
	}
	availableRelayersGauge, err := meter.Int64ObservableGauge(
		"relayer.AvailableRelayers",
		api.WithInt64Callback(func(context context.Context, result api.Int64Observer) error {
			result.Observe(*availableRelayerCount, opts)
			return nil
		}),
		api.WithDescription("Available number of relayers currently in the subset"),
	)
	if err != nil {
		return nil, err
	}

	return &MpcMetrics{
		totalRelayersGauge:     totalRelayersGauge,
		availableRelayersGauge: availableRelayersGauge,
		totalRelayerCount:      totalRelayerCount,
		availableRelayerCount:  availableRelayerCount,
	}, nil
}

func (m *MpcMetrics) TrackRelayerStatus(unavailable peer.IDSlice, all peer.IDSlice) {
	*m.totalRelayerCount = int64(len(all))
	*m.availableRelayerCount = int64(len(all) - len(unavailable))
}

// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package metrics

import (
	"context"

	"github.com/sygmaprotocol/sygma-core/observability"
	"go.opentelemetry.io/otel/attribute"
	api "go.opentelemetry.io/otel/metric"
)

type SygmaMetrics struct {
	*observability.RelayerMetrics
	*MpcMetrics
	*HostMetrics
}

// NewSygmaMetrics creates an instance of metrics
func NewSygmaMetrics(ctx context.Context, meter api.Meter, env, relayerID, version string) (*SygmaMetrics, error) {
	attributes := []attribute.KeyValue{attribute.String("relayerid", relayerID), attribute.String("env", env), attribute.String("version", version)}
	opts := api.WithAttributes(attributes...)
	relayerMetrics, err := observability.NewRelayerMetrics(ctx, meter, attributes...)
	if err != nil {
		return nil, err
	}

	mpcMetrics, err := NewMpcMetrics(ctx, meter, opts)
	if err != nil {
		return nil, err
	}

	hostMetrics, err := NewHostMetrics(ctx, meter, opts)
	if err != nil {
		return nil, err
	}

	return &SygmaMetrics{
		RelayerMetrics: relayerMetrics,
		MpcMetrics:     mpcMetrics,
		HostMetrics:    hostMetrics,
	}, nil
}

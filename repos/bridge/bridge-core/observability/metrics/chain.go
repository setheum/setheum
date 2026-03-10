package metrics

import (
	"context"
	"math/big"
	"sync"

	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

type ChainMetrics struct {
	opts metric.MeasurementOption

	blockDeltaGauge     metric.Int64ObservableGauge
	blockDeltaMap       map[uint8]*big.Int
	processedBlockMap   map[uint8]*big.Int
	processedBlockGauge metric.Int64ObservableGauge
	chainHeadMap        map[uint8]*big.Int
	chainHeadGauge      metric.Int64ObservableGauge
	lock                sync.Mutex

	gasUsedHistogram  metric.Int64Histogram
	gasPriceHistogram metric.Int64Histogram
}

// NewChainMetrics initializes metrics that provide insight into chain processing and activity
func NewChainMetrics(ctx context.Context, meter metric.Meter, opts metric.MeasurementOption) (*ChainMetrics, error) {
	blockDeltaMap := make(map[uint8]*big.Int)
	blockDeltaGauge, err := meter.Int64ObservableGauge(
		"relayer.BlockDelta",
		metric.WithInt64Callback(func(context context.Context, result metric.Int64Observer) error {
			for domainID, delta := range blockDeltaMap {
				result.Observe(delta.Int64(),
					opts,
					metric.WithAttributes(attribute.Int64("domainID", int64(domainID))),
				)
			}
			return nil
		}),
		metric.WithDescription("Difference between chain head and current indexed block per domain"),
	)
	if err != nil {
		return nil, err
	}

	chainHeadMap := make(map[uint8]*big.Int)
	chainHeadGauge, err := meter.Int64ObservableGauge(
		"relayer.ChainHead",
		metric.WithInt64Callback(func(context context.Context, result metric.Int64Observer) error {
			for domainID, head := range chainHeadMap {
				result.Observe(head.Int64(),
					opts,
					metric.WithAttributes(attribute.Int64("domainID", int64(domainID))),
				)
			}
			return nil
		}),
		metric.WithDescription("Latest block of the chain."),
	)
	if err != nil {
		return nil, err
	}

	processedBlockMap := make(map[uint8]*big.Int)
	processedBlockGauge, err := meter.Int64ObservableGauge(
		"relayer.ProcessedBlocks",
		metric.WithInt64Callback(func(context context.Context, result metric.Int64Observer) error {
			for domainID, block := range processedBlockMap {
				result.Observe(block.Int64(),
					opts,
					metric.WithAttributes(attribute.Int64("domainID", int64(domainID))),
				)
			}
			return nil
		}),
		metric.WithDescription("Latest processed block."),
	)
	if err != nil {
		return nil, err
	}

	gasUsedHistogram, err := meter.Int64Histogram(
		"relayer.GasUsed",
		metric.WithDescription("Gas used per transaction."),
		metric.WithUnit("gas"),
	)
	if err != nil {
		return nil, err
	}

	gasPriceHistogram, err := meter.Int64Histogram(
		"relayer.GasPrice",
		metric.WithDescription("Gas price distribution per transaction in gwei."),
	)
	if err != nil {
		return nil, err
	}

	return &ChainMetrics{
		opts:                opts,
		blockDeltaMap:       blockDeltaMap,
		chainHeadMap:        chainHeadMap,
		blockDeltaGauge:     blockDeltaGauge,
		chainHeadGauge:      chainHeadGauge,
		processedBlockGauge: processedBlockGauge,
		processedBlockMap:   processedBlockMap,
		gasUsedHistogram:    gasUsedHistogram,
		gasPriceHistogram:   gasPriceHistogram,
	}, nil
}

func (m *ChainMetrics) TrackBlockDelta(domainID uint8, head *big.Int, current *big.Int) {
	m.lock.Lock()
	defer m.lock.Unlock()

	m.blockDeltaMap[domainID] = new(big.Int).Sub(head, current)
	m.processedBlockMap[domainID] = new(big.Int).Set(current)
	m.chainHeadMap[domainID] = new(big.Int).Set(head)
}

func (m *ChainMetrics) TrackGasUsage(domainID uint8, gasUsed uint64, gasPrice *big.Int) {
	m.gasPriceHistogram.Record(
		context.Background(),
		gasPrice.Int64(),
		metric.WithAttributes(attribute.Int64("domainID", int64(domainID))))
	m.gasUsedHistogram.Record(
		context.Background(),
		int64(gasUsed),
		metric.WithAttributes(attribute.Int64("domainID", int64(domainID))))
}

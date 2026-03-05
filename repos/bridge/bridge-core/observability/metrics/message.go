package metrics

import (
	"context"
	"time"
	"unsafe"

	"github.com/sygmaprotocol/sygma-core/relayer/message"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

type MessageMetrics struct {
	opts metric.MeasurementOption

	totalMessageCounter      metric.Int64Counter
	failedMessageCounter     metric.Int64Counter
	successfulMessageCounter metric.Int64Counter
	latencyHistogram         metric.Float64Histogram
	transactionSizeHistogram metric.Int64Histogram
}

// NewMessageMetrics initializes metrics that insight into relayer message handling performance
func NewMessageMetrics(ctx context.Context, meter metric.Meter, opts metric.MeasurementOption) (*MessageMetrics, error) {
	totalMessageCounter, err := meter.Int64Counter(
		"relayer.TotalMessageCount",
		metric.WithDescription("Total number of messages the relayer has processed."),
	)
	if err != nil {
		return nil, err
	}
	failedMessageCounter, err := meter.Int64Counter(
		"relayer.FailedMessageCount",
		metric.WithDescription("Number of messages that have failed."),
	)
	if err != nil {
		return nil, err
	}
	successfulMessageCounter, err := meter.Int64Counter(
		"relayer.SuccessfulMessageCount",
		metric.WithDescription("Number of messages that were relayed successfully."),
	)
	if err != nil {
		return nil, err
	}

	latencyHistogram, err := meter.Float64Histogram(
		"relayer.LatencySeconds",
		metric.WithDescription("Time taken to relay messages."),
		metric.WithUnit("s"),
	)
	if err != nil {
		return nil, err
	}

	transactionSizeHistogram, err := meter.Int64Histogram(
		"relayer.MessageSizeBytes",
		metric.WithDescription("Sizes of messages processed."),
	)
	if err != nil {
		return nil, err
	}

	return &MessageMetrics{
		opts:                     opts,
		totalMessageCounter:      totalMessageCounter,
		failedMessageCounter:     failedMessageCounter,
		successfulMessageCounter: successfulMessageCounter,
		latencyHistogram:         latencyHistogram,
		transactionSizeHistogram: transactionSizeHistogram,
	}, nil
}

func (m *MessageMetrics) TrackMessages(msgs []*message.Message, status message.MessageStatus) {
	switch status {
	case message.PendingMessage:
		m.totalMessageCounter.Add(
			context.Background(),
			int64(len(msgs)),
			metric.WithAttributes(attribute.Int64("source", int64(msgs[0].Source))),
			metric.WithAttributes(attribute.Int64("destination", int64(msgs[0].Destination))))
		for _, msg := range msgs {
			m.transactionSizeHistogram.Record(
				context.Background(),
				int64(unsafe.Sizeof(msg)))
		}
	case message.FailedMessage:
		m.failedMessageCounter.Add(
			context.Background(),
			int64(len(msgs)),
			metric.WithAttributes(attribute.Int64("source", int64(msgs[0].Source))),
			metric.WithAttributes(attribute.Int64("destination", int64(msgs[0].Destination))))
	case message.SuccessfulMessage:
		m.successfulMessageCounter.Add(
			context.Background(),
			int64(len(msgs)),
			metric.WithAttributes(attribute.Int64("source", int64(msgs[0].Source))),
			metric.WithAttributes(attribute.Int64("destination", int64(msgs[0].Destination))))
		for _, msg := range msgs {
			m.latencyHistogram.Record(
				context.Background(),
				time.Since(msg.Timestamp).Seconds(),
				metric.WithAttributes(attribute.Int64("source", int64(msgs[0].Source))),
				metric.WithAttributes(attribute.Int64("destination", int64(msgs[0].Destination))))
		}
	}
}

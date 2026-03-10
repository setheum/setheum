package metrics

import (
	"context"
	"runtime"
	"runtime/debug"
	"time"

	"github.com/shirou/gopsutil/cpu"
	"github.com/shirou/gopsutil/disk"
	"github.com/shirou/gopsutil/mem"
	"github.com/shirou/gopsutil/net"
	"go.opentelemetry.io/otel/metric"
)

const (
	GC_STATS_UPDATE_PERIOD = time.Second * 10
)

type SystemMetrics struct {
	opts metric.MeasurementOption

	goRoutinesGauge        metric.Int64ObservableGauge
	totalMemoryGauge       metric.Int64ObservableGauge
	usedMemoryGauge        metric.Int64ObservableGauge
	cpuUsageGauge          metric.Float64ObservableGauge
	gcDurationHistogram    metric.Float64Histogram
	diskUsageGauge         metric.Int64ObservableGauge
	totalDiskGauge         metric.Int64ObservableGauge
	networkIOReceivedGauge metric.Int64ObservableGauge
	networkIOSentGauge     metric.Int64ObservableGauge
}

// NewSystemMetrics initializes system performance and resource utilization metrics
func NewSystemMetrics(ctx context.Context, meter metric.Meter, opts metric.MeasurementOption) (*SystemMetrics, error) {
	goRoutinesGauge, err := meter.Int64ObservableGauge(
		"relayer.GoRoutines",
		metric.WithInt64Callback(func(context context.Context, result metric.Int64Observer) error {
			result.Observe(int64(runtime.NumGoroutine()), opts)
			return nil
		}),
		metric.WithDescription("Number of Go routines running."),
	)
	if err != nil {
		return nil, err
	}

	usedMemoryGauge, err := meter.Int64ObservableGauge(
		"relayer.MemoryUsageBytes",
		metric.WithInt64Callback(func(context context.Context, result metric.Int64Observer) error {
			v, err := mem.VirtualMemory()
			if err != nil {
				return err
			}

			result.Observe(int64(v.Used), opts)
			return nil
		}),
		metric.WithDescription("Memory usage in bytes."),
	)
	if err != nil {
		return nil, err
	}
	totalMemoryGauge, err := meter.Int64ObservableGauge(
		"relayer.TotalMemoryBytes",
		metric.WithInt64Callback(func(context context.Context, result metric.Int64Observer) error {
			v, err := mem.VirtualMemory()
			if err != nil {
				return err
			}

			result.Observe(int64(v.Total), opts)
			return nil
		}),
		metric.WithDescription("Total memory in bytes."),
	)
	if err != nil {
		return nil, err
	}

	cpuUsageGauge, err := meter.Float64ObservableGauge(
		"relayer.CpuUsagePercent",
		metric.WithFloat64Callback(func(context context.Context, result metric.Float64Observer) error {
			percents, err := cpu.Percent(0, false)
			if err != nil {
				return err
			}

			result.Observe(percents[0], opts)
			return nil
		}),
		metric.WithDescription("CPU usage percent."),
	)
	if err != nil {
		return nil, err
	}

	diskUsageGauge, err := meter.Int64ObservableGauge(
		"relayer.DiskUsageBytes",
		metric.WithInt64Callback(func(context context.Context, result metric.Int64Observer) error {
			usage, err := disk.Usage("/")
			if err != nil {
				return err
			}

			result.Observe(int64(usage.Used), opts)
			return nil
		}),
		metric.WithDescription("Disk space used by the relayer in bytes."),
	)
	if err != nil {
		return nil, err
	}
	totalDiskGauge, err := meter.Int64ObservableGauge(
		"relayer.TotalDiskBytes",
		metric.WithInt64Callback(func(context context.Context, result metric.Int64Observer) error {
			usage, err := disk.Usage("/")
			if err != nil {
				return err
			}

			result.Observe(int64(usage.Total), opts)
			return nil
		}),
		metric.WithDescription("Total relayer disk space."),
	)
	if err != nil {
		return nil, err
	}

	networkIOReceivedGauge, err := meter.Int64ObservableGauge(
		"relayer.NetworkIOBytesReceived",
		metric.WithInt64Callback(func(context context.Context, result metric.Int64Observer) error {
			stat, err := net.IOCounters(false)
			if err != nil {
				return err
			}

			result.Observe(int64(stat[0].BytesRecv), opts)
			return nil
		}),
		metric.WithDescription("Total network bytes received."),
	)
	if err != nil {
		return nil, err
	}
	networkIOSentGauge, err := meter.Int64ObservableGauge(
		"relayer.NetworkIOBytesSent",
		metric.WithInt64Callback(func(context context.Context, result metric.Int64Observer) error {
			stat, err := net.IOCounters(false)
			if err != nil {
				return err
			}

			result.Observe(int64(stat[0].BytesSent), opts)
			return nil
		}),
		metric.WithDescription("Total network bytes sent."),
	)
	if err != nil {
		return nil, err
	}

	gcDurationHistogram, err := meter.Float64Histogram(
		"relayer.GcDurationSeconds",
		metric.WithDescription("Duration of garbage collection cycles."),
		metric.WithUnit("s"),
	)
	if err != nil {
		return nil, err
	}

	m := &SystemMetrics{
		opts:                   opts,
		goRoutinesGauge:        goRoutinesGauge,
		totalMemoryGauge:       totalMemoryGauge,
		usedMemoryGauge:        usedMemoryGauge,
		gcDurationHistogram:    gcDurationHistogram,
		cpuUsageGauge:          cpuUsageGauge,
		totalDiskGauge:         totalDiskGauge,
		diskUsageGauge:         diskUsageGauge,
		networkIOReceivedGauge: networkIOReceivedGauge,
		networkIOSentGauge:     networkIOSentGauge,
	}

	go m.updateGCStats(ctx)
	return m, err
}

func (m *SystemMetrics) updateGCStats(ctx context.Context) {
	ticker := time.NewTicker(GC_STATS_UPDATE_PERIOD)
	var previousPauseDuration float64
	for {
		select {
		case <-ticker.C:
			{
				var gcStats debug.GCStats
				debug.ReadGCStats(&gcStats)
				if len(gcStats.Pause) == 0 {
					continue
				}

				recentPauseDuration := gcStats.Pause[0].Seconds()
				if recentPauseDuration == previousPauseDuration {
					continue
				}

				m.gcDurationHistogram.Record(context.Background(), recentPauseDuration, m.opts)
				previousPauseDuration = recentPauseDuration
			}
		case <-ctx.Done():
			return

		}
	}
}

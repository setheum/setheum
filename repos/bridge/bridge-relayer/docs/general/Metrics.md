# Metrics
Metrics are implemented via the OpenTelemetry [stack](https://opentelemetry.io/) and exported to the Opentelemetry [collector](https://opentelemetry.io/docs/collector/) which then be configured to export to supported metrics tools like Datadog.

## Exported metrics
The following metrics are exported:
```
relayer.DepositEventCount (counter) - count of indexed deposits
relayer.ExecutionErrorCount (counter) - count of executions that failed
relayer.ExecutionLatencyPerRoute (histogram) - latency between indexing event and executing it per route
relayer.ExecutionLatency (histogram) - latency between indexing event and executing it across all routes
relayer.TotalRelayers (gauge) - number of relayers currently in the subset for MPC
relayer.availableRelayers (gauge) - number of currently available relayers from the subset
relayer.BlockDelta (gauge) - "Difference between chain head and current indexed block per domain
```

## Env variables
- SYG_RELAYER_OPENTELEMETRYCOLLECTORURL - url of the opentelemetry collector application that collects metrics
- SYG_RELAYER_ID - Set as a metrics tag (relayerid:0). Used to distinguish one Relayer from another. NOTE: should be unique and if you are planning to run Sygma relayer please agree on your relayerID with the team
- SYG_RELAYER_ENV - Set as a metrics tag (env:test). Used to distinguish Relayer environment

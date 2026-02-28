// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package health

import (
	"fmt"
	"net/http"

	"github.com/rs/zerolog/log"
)

// StartHealthEndpoint starts /health endpoint on provided port that returns ok on invocation
func StartHealthEndpoint(port uint16) {
	http.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
		_, _ = w.Write([]byte("ok"))
	})

	_ = http.ListenAndServe(fmt.Sprintf(":%d", port), nil)
	log.Info().Msgf("started /health endpoint on port %d", port)
}

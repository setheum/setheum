// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package p2p

import (
	"bufio"
	"fmt"
	"strings"
)

// ReadStream reads data from the given stream
func ReadStream(r *bufio.Reader) ([]byte, error) {
	msg, err := r.ReadString('\n')
	if err != nil {
		return []byte{}, err
	}

	if msg == "" {
		return []byte{}, fmt.Errorf("end of stream reached")
	}

	msg = strings.Trim(msg, "\n")
	return []byte(msg), nil
}

// WriteStream writes the message to stream
func WriteStream(msg []byte, w *bufio.Writer) error {
	_, err := w.WriteString(fmt.Sprintf("%s\n", string(msg[:])))
	if err != nil {
		return err
	}

	err = w.Flush()
	if err != nil {
		return fmt.Errorf("fail to flush stream: %w", err)
	}
	return nil
}

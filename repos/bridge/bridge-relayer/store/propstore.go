// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

package store

import (
	"bytes"
	"errors"
	"fmt"

	"github.com/sygmaprotocol/sygma-core/store"
	"github.com/syndtr/goleveldb/leveldb"
)

type PropStatus string

var (
	KEY                     = "source:%d:destination:%d:depositNonce:%d"
	MissingProp  PropStatus = "missing"
	PendingProp  PropStatus = "pending"
	FailedProp   PropStatus = "failed"
	ExecutedProp PropStatus = "executed"
)

type PropStore struct {
	db store.KeyValueReaderWriter
}

func NewPropStore(db store.KeyValueReaderWriter) *PropStore {
	return &PropStore{
		db: db,
	}
}

// StorePropStatus stores proposal status per proposal
func (ns *PropStore) StorePropStatus(source, destination uint8, depositNonce uint64, status PropStatus) error {
	key := bytes.Buffer{}
	keyS := fmt.Sprintf(KEY, source, destination, depositNonce)
	key.WriteString(keyS)

	err := ns.db.SetByKey(key.Bytes(), []byte(status))
	if err != nil {
		return err
	}

	return nil
}

// GetPropStatus
func (ns *PropStore) PropStatus(source, destination uint8, depositNonce uint64) (PropStatus, error) {
	key := bytes.Buffer{}
	keyS := fmt.Sprintf(KEY, source, destination, depositNonce)
	key.WriteString(keyS)

	v, err := ns.db.GetByKey(key.Bytes())
	if err != nil {
		if errors.Is(err, leveldb.ErrNotFound) {
			return MissingProp, nil
		}
		return MissingProp, err
	}

	status := PropStatus(string(v))
	return status, nil
}

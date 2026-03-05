// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package comm

import (
	"errors"
	"fmt"
	"strconv"
	"strings"
	"time"
)

// SubscriptionID is unique identifier for each subscription
// It is defined as: SessionID-MessageType-SubscriptionIdentifier
type SubscriptionID string

func NewSubscriptionID(sessionID string, msgType MessageType) SubscriptionID {
	return SubscriptionID(fmt.Sprintf("%s-%d-%d", sessionID, msgType, uint32(time.Now().UnixNano())))
}

func (sID SubscriptionID) SessionID() string {
	sessID, _, _, err := sID.Unwrap()
	if err != nil {
		return ""
	}
	return sessID
}

func (sID SubscriptionID) MessageType() MessageType {
	_, msgType, _, err := sID.Unwrap()
	if err != nil {
		return msgType
	}
	return msgType
}

func (sID SubscriptionID) SubscriptionIdentifier() string {
	_, _, subID, err := sID.Unwrap()
	if err != nil {
		return ""
	}
	return subID
}

func (sID SubscriptionID) Unwrap() (string, MessageType, string, error) {
	subIDParts := strings.Split(string(sID), "-")
	if len(subIDParts) != 3 {
		return "", Unknown, "", errors.New("invalid subscriptionID")
	}

	msgType, err := strconv.ParseInt(subIDParts[1], 10, 8)
	if err != nil {
		return "", Unknown, "", err
	}

	if msgType > int64(Unknown) {
		return "", Unknown, "", errors.New("invalid message type")
	}

	return subIDParts[0], MessageType(msgType), subIDParts[2], nil
}

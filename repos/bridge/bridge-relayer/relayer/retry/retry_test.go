package retry_test

import (
	"fmt"
	"testing"

	mock_executor "github.com/ChainSafe/sygma-relayer/chains/btc/executor/mock"
	"github.com/ChainSafe/sygma-relayer/e2e/evm"
	"github.com/ChainSafe/sygma-relayer/relayer/retry"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/ChainSafe/sygma-relayer/store"
	"github.com/ethereum/go-ethereum/common"
	"github.com/golang/mock/gomock"
	"github.com/stretchr/testify/suite"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
)

type FilterDepositsTestSuite struct {
	suite.Suite

	mockPropStorer *mock_executor.MockPropStorer
}

func TestRunRetryMessageHandlerTestSuite(t *testing.T) {
	suite.Run(t, new(FilterDepositsTestSuite))
}

func (s *FilterDepositsTestSuite) SetupTest() {
	ctrl := gomock.NewController(s.T())
	s.mockPropStorer = mock_executor.NewMockPropStorer(ctrl)
}

func (s *FilterDepositsTestSuite) Test_NoValidDeposits() {
	validResource := evm.SliceTo32Bytes(common.LeftPadBytes([]byte{3}, 31))
	invalidResource := evm.SliceTo32Bytes(common.LeftPadBytes([]byte{4}, 31))
	invalidDomain := uint8(3)
	validDomain := uint8(4)

	deposits := make(map[uint8][]*message.Message)
	deposits[validDomain] = []*message.Message{
		{
			Source:      invalidDomain,
			Destination: validDomain,
			Data: transfer.TransferMessageData{
				DepositNonce: 2,
				ResourceId:   invalidResource,
			},
		},
	}

	d, err := retry.FilterDeposits(s.mockPropStorer, deposits, validResource, validDomain)

	expectedDeposits := make([]*message.Message, 0)
	s.Nil(err)
	s.Equal(d, expectedDeposits)
}

func (s *FilterDepositsTestSuite) Test_FilterDeposits() {
	validResource := evm.SliceTo32Bytes(common.LeftPadBytes([]byte{3}, 31))
	invalidResource := evm.SliceTo32Bytes(common.LeftPadBytes([]byte{4}, 31))
	invalidDomain := uint8(3)
	validDomain := uint8(4)

	executedNonce := uint64(1)
	failedNonce := uint64(3)
	pendingNonce := uint64(4)
	failedExecutionCheckNonce := uint64(5)

	deposits := make(map[uint8][]*message.Message)
	deposits[invalidDomain] = []*message.Message{
		{
			Destination: invalidDomain,
			Data: transfer.TransferMessageData{
				DepositNonce: 1,
				ResourceId:   validResource,
			},
		},
	}
	deposits[validDomain] = []*message.Message{
		{
			Source:      invalidDomain,
			Destination: validDomain,
			Data: transfer.TransferMessageData{
				DepositNonce: executedNonce,
				ResourceId:   validResource,
			},
		},
		{
			Source:      invalidDomain,
			Destination: validDomain,
			Data: transfer.TransferMessageData{
				DepositNonce: 2,
				ResourceId:   invalidResource,
			},
		},
		{
			Source:      invalidDomain,
			Destination: validDomain,
			Data: transfer.TransferMessageData{
				DepositNonce: failedNonce,
				ResourceId:   validResource,
			},
		},
		{
			Source:      invalidDomain,
			Destination: validDomain,
			Data: transfer.TransferMessageData{
				DepositNonce: failedExecutionCheckNonce,
				ResourceId:   validResource,
			},
		},
		{
			Source:      invalidDomain,
			Destination: validDomain,
			Data: transfer.TransferMessageData{
				DepositNonce: pendingNonce,
				ResourceId:   validResource,
			},
		},
	}
	s.mockPropStorer.EXPECT().PropStatus(invalidDomain, validDomain, executedNonce).Return(store.ExecutedProp, nil)
	s.mockPropStorer.EXPECT().PropStatus(invalidDomain, validDomain, failedNonce).Return(store.FailedProp, nil)
	s.mockPropStorer.EXPECT().PropStatus(invalidDomain, validDomain, pendingNonce).Return(store.PendingProp, nil)
	s.mockPropStorer.EXPECT().PropStatus(invalidDomain, validDomain, failedExecutionCheckNonce).Return(store.PendingProp, fmt.Errorf("error"))
	s.mockPropStorer.EXPECT().StorePropStatus(invalidDomain, validDomain, pendingNonce, store.FailedProp).Return(nil)

	d, err := retry.FilterDeposits(s.mockPropStorer, deposits, validResource, validDomain)

	expectedDeposits := []*message.Message{
		{
			Source:      invalidDomain,
			Destination: validDomain,
			Data: transfer.TransferMessageData{
				DepositNonce: failedNonce,
				ResourceId:   validResource,
			},
		},
		{
			Source:      invalidDomain,
			Destination: validDomain,
			Data: transfer.TransferMessageData{
				DepositNonce: pendingNonce,
				ResourceId:   validResource,
			},
		},
	}
	s.Nil(err)
	s.Equal(d, expectedDeposits)
}

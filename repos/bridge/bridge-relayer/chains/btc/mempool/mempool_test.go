package mempool_test

import (
	"net/http"
	"net/http/httptest"
	"os"
	"testing"

	"github.com/ChainSafe/sygma-relayer/chains/btc/mempool"
	"github.com/stretchr/testify/suite"
)

func jsonFileToBytes(filename string) []byte {
	file, _ := os.ReadFile(filename)
	return file
}

type MempoolTestSuite struct {
	suite.Suite
	mempoolAPI *mempool.MempoolAPI
	server     *httptest.Server
}

func TestMempoolTestSuite(t *testing.T) {
	suite.Run(t, new(MempoolTestSuite))
}

func (s *MempoolTestSuite) SetupTest() {
	s.server = httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/address/testaddress/utxo" {
			w.WriteHeader(http.StatusOK)
			_, _ = w.Write(jsonFileToBytes("./test-data/successful-utxo.json"))
		} else if r.URL.Path == "/api/v1/fees/recommended" {
			w.WriteHeader(http.StatusOK)
			_, _ = w.Write(jsonFileToBytes("./test-data/successful-fee.json"))
		} else {
			w.WriteHeader(http.StatusNotFound)
			_, _ = w.Write([]byte("{\"status\":\"Not found\"}"))
		}
	}))
	s.mempoolAPI = mempool.NewMempoolAPI(s.server.URL)
}
func (s *MempoolTestSuite) TeardownTest() {
	s.server.Close()
}

func (s *MempoolTestSuite) Test_Utxo_SuccessfulFetch() {
	utxos, err := s.mempoolAPI.Utxos("testaddress")

	s.Nil(err)
	s.Equal(utxos, []mempool.Utxo{
		{
			TxID:  "28154e2008912d27978225c096c22ffe2ea65e1d55bf440ee41c21f9489c7fe1",
			Vout:  0,
			Value: 11197,
			Status: mempool.Status{
				Confirmed:   true,
				BlockHeight: 2812826,
				BlockHash:   "000000000000001a01d4058773384f2c23aed5a7e5ede252f99e290fa58324a3",
				BlockTime:   1715083122,
			},
		},
		{
			TxID:  "28154e2008912d27978225c096c22ffe2ea65e1d55bf440ee41c21f9489c7fe2",
			Vout:  0,
			Value: 11197,
			Status: mempool.Status{
				Confirmed:   true,
				BlockHeight: 2812826,
				BlockHash:   "000000000000001a01d4058773384f2c23aed5a7e5ede252f99e290fa58324a3",
				BlockTime:   1715083122,
			},
		},
		{
			TxID:  "28154e2008912d27978225c096c22ffe2ea65e1d55bf440ee41c21f9489c7fe9",
			Vout:  0,
			Value: 11198,
			Status: mempool.Status{
				Confirmed:   true,
				BlockHeight: 2812827,
				BlockHash:   "000000000000001a01d4058773384f2c23aed5a7e5ede252f99e290fa58324a5",
				BlockTime:   1715083123,
			},
		},
	})
}

func (s *MempoolTestSuite) Test_Utxo_NotFound() {
	_, err := s.mempoolAPI.Utxos("invalid")

	s.NotNil(err)
}

func (s *MempoolTestSuite) Test_RecommendedFee_SuccessfulFetch() {
	recommendedFee, err := s.mempoolAPI.RecommendedFee()

	s.Nil(err)
	s.Equal(recommendedFee, &mempool.Fee{
		FastestFee:  1,
		HalfHourFee: 2,
		HourFee:     3,
		EconomyFee:  4,
		MinimumFee:  5,
	})

}

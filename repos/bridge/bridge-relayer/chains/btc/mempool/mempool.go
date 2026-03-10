package mempool

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"sort"
)

type Status struct {
	Confirmed   bool
	BlockHeight uint64 `json:"block_height"`
	BlockHash   string `json:"block_hash"`
	BlockTime   uint64 `json:"block_time"`
}

type Utxo struct {
	TxID   string `json:"txid"`
	Vout   uint32 `json:"vout"`
	Value  uint64 `json:"value"`
	Status Status `json:"status"`
}

type Fee struct {
	FastestFee  uint64
	HalfHourFee uint64
	MinimumFee  uint64
	EconomyFee  uint64
	HourFee     uint64
}

type MempoolAPI struct {
	url string
}

func NewMempoolAPI(url string) *MempoolAPI {
	return &MempoolAPI{
		url: url,
	}
}

func (a *MempoolAPI) RecommendedFee() (*Fee, error) {
	resp, err := http.Get(fmt.Sprintf("%s/api/v1/fees/recommended", a.url))
	if err != nil {
		return nil, err
	}

	var fee *Fee
	defer resp.Body.Close()
	data, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}
	err = json.Unmarshal(data, &fee)
	if err != nil {
		return nil, err
	}

	return fee, nil
}

func (a *MempoolAPI) Utxos(address string) ([]Utxo, error) {
	resp, err := http.Get(fmt.Sprintf("%s/api/address/%s/utxo", a.url, address))
	if err != nil {
		return nil, err
	}

	var utxos []Utxo
	defer resp.Body.Close()
	data, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}
	err = json.Unmarshal(data, &utxos)
	if err != nil {
		return nil, err
	}
	sort.Slice(utxos, func(i int, j int) bool {
		if utxos[i].Status.BlockTime == utxos[j].Status.BlockTime {
			return utxos[i].TxID < utxos[j].TxID
		} else {
			return utxos[i].Status.BlockTime < utxos[j].Status.BlockTime
		}
	})

	return utxos, nil
}

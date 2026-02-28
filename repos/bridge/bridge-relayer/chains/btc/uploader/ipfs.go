package uploader

import (
	"bytes"
	"encoding/json"
	"errors"
	"io"
	"mime/multipart"
	"net/http"
	"time"

	"github.com/rs/zerolog/log"

	"github.com/ChainSafe/sygma-relayer/config/relayer"
	"github.com/cenkalti/backoff/v4"
)

type Uploader interface {
	Upload(proposals []map[string]interface{}) (string, error)
}

type IPFSUploader struct {
	config relayer.UploaderConfig
}

func NewIPFSUploader(config relayer.UploaderConfig) *IPFSUploader {
	return &IPFSUploader{config: config}
}

type IPFSResponse struct {
	IpfsHash string `json:"IpfsHash"`
}

func (i *IPFSUploader) Upload(dataToUpload []map[string]interface{}) (string, error) {
	jsonData, err := json.Marshal(dataToUpload)
	if err != nil {
		return "", err
	}

	body := new(bytes.Buffer)
	writer := multipart.NewWriter(body)
	part, err := writer.CreateFormFile("file", "metadata.json")
	if err != nil {
		return "", err
	}
	_, err = part.Write(jsonData)
	if err != nil {
		return "", err
	}
	writer.Close()

	req, err := http.NewRequest("POST", i.config.URL, body)
	if err != nil {
		return "", err
	}

	req.Header.Add("Authorization", "Bearer "+i.config.AuthToken)
	req.Header.Add("Content-Type", writer.FormDataContentType())

	var ipfsResponse IPFSResponse

	// Define the operation to be retried
	operation := func() error {
		return i.performRequest(req, &ipfsResponse)
	}
	expBackoff := backoff.NewExponentialBackOff()
	expBackoff.MaxElapsedTime = i.config.MaxElapsedTime

	notify := func(err error, duration time.Duration) {
		log.Warn().Err(err).Msg("Unable to upload metadata to ipfs")
	}

	err = backoff.RetryNotify(operation, backoff.WithMaxRetries(expBackoff, i.config.MaxRetries), notify)
	if err != nil {
		return "", err
	}

	return ipfsResponse.IpfsHash, nil
}

func (i *IPFSUploader) performRequest(req *http.Request, ipfsResponse *IPFSResponse) error {
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != 200 {
		return errors.New("received non-200 status code")
	}

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return err
	}

	if err := json.Unmarshal(respBody, &ipfsResponse); err != nil {
		return err
	}

	return nil
}

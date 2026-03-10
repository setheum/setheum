// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package topology

import (
	"bytes"
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
)

type AESEncryption struct {
	block cipher.Block
}

func NewAESEncryption(key []byte) (*AESEncryption, error) {
	block, err := aes.NewCipher(key)
	if err != nil {
		return nil, err
	}

	return &AESEncryption{
		block: block,
	}, nil
}

func (ae *AESEncryption) Decrypt(ct []byte) []byte {
	iv := ct[:aes.BlockSize]
	stream := cipher.NewCTR(ae.block, iv)
	dst := make([]byte, len(ct[aes.BlockSize:]))
	stream.XORKeyStream(dst, ct[aes.BlockSize:])
	return dst
}

// Encrypt is a function that encrypts provided bytes with AES in CTR mode
// Returned value is iv + ct
func (ae *AESEncryption) Encrypt(data []byte) ([]byte, error) {
	dst := make([]byte, len(data))
	iv := make([]byte, 16)
	_, err := rand.Read(iv)
	if err != nil {
		return nil, err
	}
	stream := cipher.NewCTR(ae.block, iv)
	stream.XORKeyStream(dst, data)

	ct := bytes.NewBuffer(iv)
	ct.Write(dst)
	return ct.Bytes(), nil
}

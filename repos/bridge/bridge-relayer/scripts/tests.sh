#!/usr/bin/env bash
# The Licensed Work is (c) 2022 Sygma
# SPDX-License-Identifier: LGPL-3.0-only


CVPKG=$(go list ./... | grep -v 'e2e\|generated\|bindata\|mock\|main.go\|' | tr '\n' ',')
go test -timeout 30m -coverpkg=$CVPKG -coverprofile=cover.out -p=1 $(go list ./... | grep -v 'cbcli\|e2e')

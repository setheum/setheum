// Copyright 2018-2021 @setheum.js/api authors & contributors
// SPDX-License-Identifier: Apache-2.0

const config = require('@open-web3/dev-config/config/jest.cjs');

module.exports = Object.assign({}, config, {
  moduleNameMapper: {
    '@setheum.js/api-derive(.*)$': '<rootDir>/packages/api-derive/src/$1',
    '@setheum.js/api(.*)$': '<rootDir>/packages/api/src/$1',
    '@setheum.js/types(.*)$': '<rootDir>/packages/types/src/$1',
    '@setheum.js/type-definitions(.*)$': '<rootDir>/packages/type-definitions/src/$1',
    '@setheum.js/sdk-core(.*)$': '<rootDir>/packages/sdk-core/src/$1',
    '@setheum.js/sdk-swap(.*)$': '<rootDir>/packages/sdk-swapcore/src/$1'
  },
  modulePathIgnorePatterns: [
    '<rootDir>/build',
    '<rootDir>/packages/api/build',
    '<rootDir>/packages/types/build',
    '<rootDir>/packages/api-derive/build',
    '<rootDir>/packages/type-definitions/build',
    '<rootDir>/packages/sdk-core/build',
    '<rootDir>/packages/sdk-swap/build'
  ],
  transformIgnorePatterns: ['/node_modules/(?!@polkadot|@babel/runtime/helpers/esm/)']
});

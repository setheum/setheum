# Changelog

## [2.4.2](https://github.com/sygmaprotocol/sygma-x-solidity/compare/v2.4.1...v2.4.2) (2024-03-25)


### Bug Fixes

* remove ignore optional when releasing ([#50](https://github.com/sygmaprotocol/sygma-x-solidity/issues/50)) ([8630892](https://github.com/sygmaprotocol/sygma-x-solidity/commit/86308923b57a68eb7583ab4e7de9b32e4ad22989))

## [2.4.1](https://github.com/sygmaprotocol/sygma-x-solidity/compare/v2.4.0...v2.4.1) (2024-03-25)


### Bug Fixes

* lockfile ([#48](https://github.com/sygmaprotocol/sygma-x-solidity/issues/48)) ([040485a](https://github.com/sygmaprotocol/sygma-x-solidity/commit/040485aefe48fcff2d06ecca8d3012063d33a54d))

## [2.4.0](https://github.com/sygmaprotocol/sygma-x-solidity/compare/v2.3.0...v2.4.0) (2024-03-25)


### Features

* add coverage, gas reporter, linting and prettier ([#12](https://github.com/sygmaprotocol/sygma-x-solidity/issues/12)) ([3ca83b5](https://github.com/sygmaprotocol/sygma-x-solidity/commit/3ca83b519240ffcf17f3af047dc6e5b069a4bf5c))
* add migration scripts with automatic contract verification ([#26](https://github.com/sygmaprotocol/sygma-x-solidity/issues/26)) ([bd43d11](https://github.com/sygmaprotocol/sygma-x-solidity/commit/bd43d1138b38328267f2bfdb65a37817f24e3286))
* add security model to fee handlers and fees mapping ([#30](https://github.com/sygmaprotocol/sygma-x-solidity/issues/30)) ([f69e45b](https://github.com/sygmaprotocol/sygma-x-solidity/commit/f69e45b8cc9268523063994ff60975334ba05c37))
* add workflow for publishing to npm ([#29](https://github.com/sygmaprotocol/sygma-x-solidity/issues/29)) ([9cafa97](https://github.com/sygmaprotocol/sygma-x-solidity/commit/9cafa97f5b464f0e20ffd95a71053247fb95249c))
* expose method for marking nonce as used ([#27](https://github.com/sygmaprotocol/sygma-x-solidity/issues/27)) ([3e4748f](https://github.com/sygmaprotocol/sygma-x-solidity/commit/3e4748f9a9b35df33474766dd4845d1105b6e2c5))
* Implement fee whitelist ([#203](https://github.com/sygmaprotocol/sygma-x-solidity/issues/203)) ([4463bcb](https://github.com/sygmaprotocol/sygma-x-solidity/commit/4463bcb03fd046875e8109fa5e9266ffdc304015))
* implement proof verification ([#20](https://github.com/sygmaprotocol/sygma-x-solidity/issues/20)) ([d461322](https://github.com/sygmaprotocol/sygma-x-solidity/commit/d461322aa03ec13766f74b3f10ab2082f6a798e4))
* introduce multiple verifiers per security model ([#23](https://github.com/sygmaprotocol/sygma-x-solidity/issues/23)) ([308b918](https://github.com/sygmaprotocol/sygma-x-solidity/commit/308b918baf8213e7ea2e243c944b0d7bf999c2cf))
* limit permissionless generic call gas usage ([#200](https://github.com/sygmaprotocol/sygma-x-solidity/issues/200)) ([d7823d7](https://github.com/sygmaprotocol/sygma-x-solidity/commit/d7823d7fc1879718387355b8f687e12bd587aa9c))
* percentage based fee handler ([#194](https://github.com/sygmaprotocol/sygma-x-solidity/issues/194)) ([26dc82a](https://github.com/sygmaprotocol/sygma-x-solidity/commit/26dc82a1bd129de968fa2244b7ce36542b46cb27))
* refactor handlers so they return deposit data intead of handler response ([#15](https://github.com/sygmaprotocol/sygma-x-solidity/issues/15)) ([d402891](https://github.com/sygmaprotocol/sygma-x-solidity/commit/d40289158d4bf4f23213619b4567ef7962944e8f))
* separate deposit execution logic ([#14](https://github.com/sygmaprotocol/sygma-x-solidity/issues/14)) ([4ea35ff](https://github.com/sygmaprotocol/sygma-x-solidity/commit/4ea35ff8886a375941d4ea565ca9247b88360aa4))
* spectre proxy ([#18](https://github.com/sygmaprotocol/sygma-x-solidity/issues/18)) ([609ca8b](https://github.com/sygmaprotocol/sygma-x-solidity/commit/609ca8be426721c52ae8f5c25e7aa642b28b5b23))
* update spectre proxy according to the latest spectre contract ([#38](https://github.com/sygmaprotocol/sygma-x-solidity/issues/38)) ([9721143](https://github.com/sygmaprotocol/sygma-x-solidity/commit/9721143069c0d7a976caa5629a4b9ce03aaf200f))


### Bug Fixes

* add missing custom errors ([#43](https://github.com/sygmaprotocol/sygma-x-solidity/issues/43)) ([34e2440](https://github.com/sygmaprotocol/sygma-x-solidity/commit/34e2440b744f2bfd4857b163d58f7f32bb9c28c5))
* add renounce admin zero address check ([#44](https://github.com/sygmaprotocol/sygma-x-solidity/issues/44)) ([68a6549](https://github.com/sygmaprotocol/sygma-x-solidity/commit/68a6549ac5c1cdd17f88bb93390bcbe85107baff))
* add zero address constructor checks ([#39](https://github.com/sygmaprotocol/sygma-x-solidity/issues/39)) ([e6b5cf8](https://github.com/sygmaprotocol/sygma-x-solidity/commit/e6b5cf8909fb697eba8346ebebb195496ea76429))
* change depth according to deneb ([#45](https://github.com/sygmaprotocol/sygma-x-solidity/issues/45)) ([dac584d](https://github.com/sygmaprotocol/sygma-x-solidity/commit/dac584dd0a87bf779384d304943bc00c19fe4586))
* deploying local network 2 ([#196](https://github.com/sygmaprotocol/sygma-x-solidity/issues/196)) ([a67d5d1](https://github.com/sygmaprotocol/sygma-x-solidity/commit/a67d5d1c3db9aab609db055dd48fdf93e293e0ad))
* proof lenght check ([#37](https://github.com/sygmaprotocol/sygma-x-solidity/issues/37)) ([6e1d38d](https://github.com/sygmaprotocol/sygma-x-solidity/commit/6e1d38d0d3fec0a80be329cc4a56015dc96698fb))
* revert with custom error for 0 conversion amounts ([#36](https://github.com/sygmaprotocol/sygma-x-solidity/issues/36)) ([cb2d7ae](https://github.com/sygmaprotocol/sygma-x-solidity/commit/cb2d7ae43246453633bed02ae27fcbdaad951f79))
* update node version ([#33](https://github.com/sygmaprotocol/sygma-x-solidity/issues/33)) ([64245f7](https://github.com/sygmaprotocol/sygma-x-solidity/commit/64245f73dcf5a7e1aa1117caa0b6f5c5184b20eb))


### Miscellaneous

* add description about transferHashes mapping usage ([#40](https://github.com/sygmaprotocol/sygma-x-solidity/issues/40)) ([6c3233c](https://github.com/sygmaprotocol/sygma-x-solidity/commit/6c3233c817d3cab4340ab81998e3d068e4b1ce6f))
* change package name ([#47](https://github.com/sygmaprotocol/sygma-x-solidity/issues/47)) ([b6e8c5d](https://github.com/sygmaprotocol/sygma-x-solidity/commit/b6e8c5db6e1c39161b256e291754f193366c3f9b))
* gas optimizations ([#35](https://github.com/sygmaprotocol/sygma-x-solidity/issues/35)) ([37517c1](https://github.com/sygmaprotocol/sygma-x-solidity/commit/37517c1441568549d86072dbeace1a5ca50571b6))
* introduce security indexes per source domain ([#22](https://github.com/sygmaprotocol/sygma-x-solidity/issues/22)) ([ffae162](https://github.com/sygmaprotocol/sygma-x-solidity/commit/ffae1621ed2e0213a7ec029110052b75c444299a))
* **master:** release 2.4.0 ([#193](https://github.com/sygmaprotocol/sygma-x-solidity/issues/193)) ([bb376f4](https://github.com/sygmaprotocol/sygma-x-solidity/commit/bb376f4e18121bcc118690ff90676dcc132f0fe4))
* **master:** release 2.4.1 ([#197](https://github.com/sygmaprotocol/sygma-x-solidity/issues/197)) ([0cc78cf](https://github.com/sygmaprotocol/sygma-x-solidity/commit/0cc78cf1e8ce1c2f9d8ed910c5e789f324f8e032))
* permission generic handlers ([#195](https://github.com/sygmaprotocol/sygma-x-solidity/issues/195)) ([6eb7041](https://github.com/sygmaprotocol/sygma-x-solidity/commit/6eb704180dd8344f47f5b0d039612c673456de59))
* remove unused imports, excess casting and fix func visibility ([#41](https://github.com/sygmaprotocol/sygma-x-solidity/issues/41)) ([556d31a](https://github.com/sygmaprotocol/sygma-x-solidity/commit/556d31a2b3761203f7aea73871464b38599ccc69))
* replace custom AccessControl implementation with OZ's ([#34](https://github.com/sygmaprotocol/sygma-x-solidity/issues/34)) ([8342d3d](https://github.com/sygmaprotocol/sygma-x-solidity/commit/8342d3d537f87979d4dd4cc800cd84e4d1489bb9))
* replace require statements with custom errors ([#42](https://github.com/sygmaprotocol/sygma-x-solidity/issues/42)) ([30cb6c4](https://github.com/sygmaprotocol/sygma-x-solidity/commit/30cb6c431c34ad265f0f5ad95498be57863dfd11))
* update devnet, testnet & mainnet migrations config files ([#190](https://github.com/sygmaprotocol/sygma-x-solidity/issues/190)) ([fb37549](https://github.com/sygmaprotocol/sygma-x-solidity/commit/fb37549132519f84c7c284d99c92579f02e1f6b7))
* update license ([#192](https://github.com/sygmaprotocol/sygma-x-solidity/issues/192)) ([faf8305](https://github.com/sygmaprotocol/sygma-x-solidity/commit/faf83050bc6888c054134481d1883a7c15f5090a))
* update migrations to support percetange fee handler + flow improvements ([#198](https://github.com/sygmaprotocol/sygma-x-solidity/issues/198)) ([746d51e](https://github.com/sygmaprotocol/sygma-x-solidity/commit/746d51e108fb3b03616ba533b2dbde96b4c2bbdc))

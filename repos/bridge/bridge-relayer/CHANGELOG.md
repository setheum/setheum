# Changelog

## [2.6.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v2.5.0...v2.6.0) (2024-12-04)


### Features

* cross network retry ([#337](https://github.com/sygmaprotocol/sygma-relayer/issues/337)) ([eb43206](https://github.com/sygmaprotocol/sygma-relayer/commit/eb432060ef3fa823437e5039137320ecbbe84cc3))
* metrics ([#380](https://github.com/sygmaprotocol/sygma-relayer/issues/380)) ([03ec27c](https://github.com/sygmaprotocol/sygma-relayer/commit/03ec27cc9b9ab754279a3c6e7cdacc52ce7606bd))


### Bug Fixes

* add optional revert gas to max fee set by user ([#388](https://github.com/sygmaprotocol/sygma-relayer/issues/388)) ([aacf99a](https://github.com/sygmaprotocol/sygma-relayer/commit/aacf99a30ec2b3d8ba10469b3f000e4aad8b9d6b))
* v1 retry signature ([#386](https://github.com/sygmaprotocol/sygma-relayer/issues/386)) ([ae06d67](https://github.com/sygmaprotocol/sygma-relayer/commit/ae06d6776ef302cf95d25dbc2afaaced476de7f3))


### Miscellaneous

* added loki logs to example docker-compose ([#385](https://github.com/sygmaprotocol/sygma-relayer/issues/385)) ([2f772c2](https://github.com/sygmaprotocol/sygma-relayer/commit/2f772c26e1949ba6258689d77b28beb632bebd79))

## [2.5.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v2.4.1...v2.5.0) (2024-10-23)


### Features

* match configs by domainID ([#375](https://github.com/sygmaprotocol/sygma-relayer/issues/375)) ([f695875](https://github.com/sygmaprotocol/sygma-relayer/commit/f6958751cf3838aedca255d8469525fa3056f2ce))


### Bug Fixes

* enable setting custom transfer gas per chain ([#376](https://github.com/sygmaprotocol/sygma-relayer/issues/376)) ([0b38483](https://github.com/sygmaprotocol/sygma-relayer/commit/0b3848362c193ba4463dd14e6a52748063acd1b7))
* panic on shared config network fetching ([#378](https://github.com/sygmaprotocol/sygma-relayer/issues/378)) ([6cd6397](https://github.com/sygmaprotocol/sygma-relayer/commit/6cd6397f02c63589e4fdb4d8326a45283a69e029))


### Miscellaneous

* Access Secrets based on specific environments ([#367](https://github.com/sygmaprotocol/sygma-relayer/issues/367)) ([eed1e97](https://github.com/sygmaprotocol/sygma-relayer/commit/eed1e97d4cb09c9c2920aedcdec3801157629e94))
* Update version of Substrate pallet  ([#377](https://github.com/sygmaprotocol/sygma-relayer/issues/377)) ([4a3613b](https://github.com/sygmaprotocol/sygma-relayer/commit/4a3613b7e6e853c6c83c57c830fd78b2a49830b3))

## [2.4.1](https://github.com/sygmaprotocol/sygma-relayer/compare/v2.4.0...v2.4.1) (2024-09-18)


### Bug Fixes

* add generic call gas limit buffer ([#364](https://github.com/sygmaprotocol/sygma-relayer/issues/364)) ([3f01dae](https://github.com/sygmaprotocol/sygma-relayer/commit/3f01dae4cc69f46afd5c308c463ed6ae42a9f5f6))

## [2.4.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v2.3.0...v2.4.0) (2024-09-16)


### Features

* treat native handler as erc20 ([#361](https://github.com/sygmaprotocol/sygma-relayer/issues/361)) ([d292ca0](https://github.com/sygmaprotocol/sygma-relayer/commit/d292ca0909459d2ad272bc76756996ed0333be63))


### Bug Fixes

* remove invalid frost resharing ([#363](https://github.com/sygmaprotocol/sygma-relayer/issues/363)) ([491f4f4](https://github.com/sygmaprotocol/sygma-relayer/commit/491f4f4367b5db716898c1157a1f2d66f0d5559a))
* remove peer as well as addresses from peerstore ([#359](https://github.com/sygmaprotocol/sygma-relayer/issues/359)) ([fca7423](https://github.com/sygmaprotocol/sygma-relayer/commit/fca74235f8a1b10e123e41d580715fa889fbc3fe))

## [2.3.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v2.2.0...v2.3.0) (2024-09-03)


### Features

* erc20 additional data ([#356](https://github.com/sygmaprotocol/sygma-relayer/issues/356)) ([74f3c5b](https://github.com/sygmaprotocol/sygma-relayer/commit/74f3c5b30e7cdc62600bffba45dbb6416bea5e09))
* upload btc execution metadata to ipfs ([#339](https://github.com/sygmaprotocol/sygma-relayer/issues/339)) ([ff55000](https://github.com/sygmaprotocol/sygma-relayer/commit/ff55000d8e78f2138fd3d5849a8ae110429508cd))


### Bug Fixes

* frost key resharing ([#346](https://github.com/sygmaprotocol/sygma-relayer/issues/346)) ([7b35e11](https://github.com/sygmaprotocol/sygma-relayer/commit/7b35e11a5eaaa5d76ed01a5030432a70ee69d0c8))
* return error when upload to ipfs fails ([#350](https://github.com/sygmaprotocol/sygma-relayer/issues/350)) ([8aff649](https://github.com/sygmaprotocol/sygma-relayer/commit/8aff6493514ce260563fe3d143079ca88b36ce29))
* treat coordinator missing as a warn log ([#352](https://github.com/sygmaprotocol/sygma-relayer/issues/352)) ([25c41f5](https://github.com/sygmaprotocol/sygma-relayer/commit/25c41f5ba0df0801636ccd8d3496f41e466dacab))


### Miscellaneous

* add ipfs node to docker-compose ([#351](https://github.com/sygmaprotocol/sygma-relayer/issues/351)) ([09039b5](https://github.com/sygmaprotocol/sygma-relayer/commit/09039b53c05e6ded762650eae31086a28e8df109))
* update fees docs ([#343](https://github.com/sygmaprotocol/sygma-relayer/issues/343)) ([6cfbbcc](https://github.com/sygmaprotocol/sygma-relayer/commit/6cfbbccbd1e0e699433a767f44870a45938e3a29))

## [2.2.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v2.1.2...v2.2.0) (2024-07-24)


### Features

* btc fee collection ([#299](https://github.com/sygmaprotocol/sygma-relayer/issues/299)) ([9eeff52](https://github.com/sygmaprotocol/sygma-relayer/commit/9eeff5228db1c157ac53a91ea97732408437345b))
* enable starting multiple tss processes with the same peer subset ([#331](https://github.com/sygmaprotocol/sygma-relayer/issues/331)) ([f65b16f](https://github.com/sygmaprotocol/sygma-relayer/commit/f65b16f9388e365a656817b526de5d8b1c39643a))
* unlock pending btc proposals when doing retry ([#319](https://github.com/sygmaprotocol/sygma-relayer/issues/319)) ([5c67c79](https://github.com/sygmaprotocol/sygma-relayer/commit/5c67c793902c2ed121cccab6433bc8a1aa9aba84))


### Bug Fixes

* add fee estimate when calculating btc inputs ([#311](https://github.com/sygmaprotocol/sygma-relayer/issues/311)) ([0ca3a1b](https://github.com/sygmaprotocol/sygma-relayer/commit/0ca3a1b93d5d885a8c2686e86582df4413ea174b))
* bad non zero check ([#315](https://github.com/sygmaprotocol/sygma-relayer/issues/315)) ([90ab5eb](https://github.com/sygmaprotocol/sygma-relayer/commit/90ab5ebe26ee157aab7999c053b4a08f026afcaf))
* bump core to fix panic on invalid domain ([#328](https://github.com/sygmaprotocol/sygma-relayer/issues/328)) ([34830ae](https://github.com/sygmaprotocol/sygma-relayer/commit/34830aef2ddad502fea96ac3905762f7ddfba120))
* enforce proposal execution per resource ([#309](https://github.com/sygmaprotocol/sygma-relayer/issues/309)) ([49a0777](https://github.com/sygmaprotocol/sygma-relayer/commit/49a07772563f1a579035f979ae6db6511f803e2b))
* fix typos ([#313](https://github.com/sygmaprotocol/sygma-relayer/issues/313)) ([a3c54af](https://github.com/sygmaprotocol/sygma-relayer/commit/a3c54afd957bae2d1e918846a1319059811b6f12))
* handle msg.To being empty properly ([#312](https://github.com/sygmaprotocol/sygma-relayer/issues/312)) ([68a3acc](https://github.com/sygmaprotocol/sygma-relayer/commit/68a3accae212d7712b94079f8c33133572d8c479))
* ignore refresh errors to prevent infinite loops ([#332](https://github.com/sygmaprotocol/sygma-relayer/issues/332)) ([496e9ae](https://github.com/sygmaprotocol/sygma-relayer/commit/496e9aed26cbb041f92c34ee9ea39f3f939865a7))
* is proposal executed nonce ([#335](https://github.com/sygmaprotocol/sygma-relayer/issues/335)) ([a966428](https://github.com/sygmaprotocol/sygma-relayer/commit/a96642887db523557f7d19506df481b24225d4b1))
* possible race condition ([#316](https://github.com/sygmaprotocol/sygma-relayer/issues/316)) ([b9b5341](https://github.com/sygmaprotocol/sygma-relayer/commit/b9b5341f61b7cdf1ca2c408d1f05074d979a4e80))
* post audit fixes ([#329](https://github.com/sygmaprotocol/sygma-relayer/issues/329)) ([d23d61d](https://github.com/sygmaprotocol/sygma-relayer/commit/d23d61d4be792c030a74c832547865e2e8c05035))
* reduce tx nonce collision chances ([#314](https://github.com/sygmaprotocol/sygma-relayer/issues/314)) ([5431db5](https://github.com/sygmaprotocol/sygma-relayer/commit/5431db55c1dcb47b41616032ed0cc376d0fcf78e))
* remove ECDSA peer ([#324](https://github.com/sygmaprotocol/sygma-relayer/issues/324)) ([3c4cbaa](https://github.com/sygmaprotocol/sygma-relayer/commit/3c4cbaa5a5bae5d14af2f3e343445dc56c17d84f))
* schnorr signature hash missing leading zeroes ([#341](https://github.com/sygmaprotocol/sygma-relayer/issues/341)) ([34f52b1](https://github.com/sygmaprotocol/sygma-relayer/commit/34f52b17c314aca63e4d32c7988eb1e7049eb576))
* sort utxos by oldest to prevent mismatched messages ([#317](https://github.com/sygmaprotocol/sygma-relayer/issues/317)) ([033b58a](https://github.com/sygmaprotocol/sygma-relayer/commit/033b58a92236bf8c97198912cbfb4c0ec487e7b4))
* update key threshold when doing frost resharing ([#333](https://github.com/sygmaprotocol/sygma-relayer/issues/333)) ([f1e6db1](https://github.com/sygmaprotocol/sygma-relayer/commit/f1e6db13998487c6af77097615f68344c84c8f53))
* update sygma-core version ([#321](https://github.com/sygmaprotocol/sygma-relayer/issues/321)) ([94429d4](https://github.com/sygmaprotocol/sygma-relayer/commit/94429d42b26726ccfa8414a24dacdc61b51f5783))


### Miscellaneous

* add btc deposit format explanation ([#318](https://github.com/sygmaprotocol/sygma-relayer/issues/318)) ([080554c](https://github.com/sygmaprotocol/sygma-relayer/commit/080554c24662acb74153789c343dcc8fb57b5e5c))
* bump polkadot libs ([#330](https://github.com/sygmaprotocol/sygma-relayer/issues/330)) ([1119f23](https://github.com/sygmaprotocol/sygma-relayer/commit/1119f231542225bc64c2e4511e35e63d382fe11c))

## [2.1.2](https://github.com/sygmaprotocol/sygma-relayer/compare/v2.1.1...v2.1.2) (2024-06-21)


### Bug Fixes

* Stop tssProcess only once in coordinator.execute function ([#276](https://github.com/sygmaprotocol/sygma-relayer/issues/276)) ([d2e83e9](https://github.com/sygmaprotocol/sygma-relayer/commit/d2e83e92fd8ec4c0590142479ba3e8c8bf42f626))


### Miscellaneous

* remove keyshare check ([#310](https://github.com/sygmaprotocol/sygma-relayer/issues/310)) ([4f50c7c](https://github.com/sygmaprotocol/sygma-relayer/commit/4f50c7c1c578faaef96bf1bb4302c77ffa4c571a))

## [2.1.1](https://github.com/sygmaprotocol/sygma-relayer/compare/v2.1.0...v2.1.1) (2024-06-14)


### Bug Fixes

* add timeout for frost outbound messages ([#301](https://github.com/sygmaprotocol/sygma-relayer/issues/301)) ([53c63a0](https://github.com/sygmaprotocol/sygma-relayer/commit/53c63a054948c9dec0d94fdcf17b4ee20c946e17))


### Miscellaneous

* add e2e tests btc -&gt; evm transfer ([#298](https://github.com/sygmaprotocol/sygma-relayer/issues/298)) ([79777bd](https://github.com/sygmaprotocol/sygma-relayer/commit/79777bdaab8e86d337bfc03069803b4745b5316f))

## [2.1.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v2.0.4...v2.1.0) (2024-06-11)


### Features

* bitcoin ([#296](https://github.com/sygmaprotocol/sygma-relayer/issues/296)) ([ff09579](https://github.com/sygmaprotocol/sygma-relayer/commit/ff09579a5c384f010ecd57657663c29ef26573ea))

## [2.0.4](https://github.com/sygmaprotocol/sygma-relayer/compare/v2.0.3...v2.0.4) (2024-05-29)


### Bug Fixes

* refresh coordinator being outside the peerstore ([#283](https://github.com/sygmaprotocol/sygma-relayer/issues/283)) ([9dfee13](https://github.com/sygmaprotocol/sygma-relayer/commit/9dfee131d9bfcd69dec45b4051f3bbc655fe0971))


### Miscellaneous

* reduced devnet to one nodes per region ([#271](https://github.com/sygmaprotocol/sygma-relayer/issues/271)) ([d84aae3](https://github.com/sygmaprotocol/sygma-relayer/commit/d84aae31ea932e56516548026ffb8637f8e9f182))
* visibility of image versions ([#292](https://github.com/sygmaprotocol/sygma-relayer/issues/292)) ([ce8a986](https://github.com/sygmaprotocol/sygma-relayer/commit/ce8a98679176c6db8647da1997917f529b916d0a))

## [2.0.3](https://github.com/sygmaprotocol/sygma-relayer/compare/v2.0.2...v2.0.3) (2024-05-20)


### Miscellaneous

* Add keygen CLI ([#278](https://github.com/sygmaprotocol/sygma-relayer/issues/278)) ([9d9a113](https://github.com/sygmaprotocol/sygma-relayer/commit/9d9a113df03a417e715f7e59ff73665b7456ba31))
* remove relayers from devnet pipeline ([#280](https://github.com/sygmaprotocol/sygma-relayer/issues/280)) ([14fdb0d](https://github.com/sygmaprotocol/sygma-relayer/commit/14fdb0dc6d0fd7e19e93b1f08e75c7a335b5cafa))

## [2.0.2](https://github.com/sygmaprotocol/sygma-relayer/compare/v2.0.1...v2.0.2) (2024-04-18)


### Bug Fixes

* use existing message channel for substrate network event handlers ([#272](https://github.com/sygmaprotocol/sygma-relayer/issues/272)) ([a592ef7](https://github.com/sygmaprotocol/sygma-relayer/commit/a592ef7fe06ffbad3efd6008430cd9d0da874264))

## [2.0.1](https://github.com/sygmaprotocol/sygma-relayer/compare/v2.0.0...v2.0.1) (2024-04-15)


### Bug Fixes

* revert starting block changes ([#269](https://github.com/sygmaprotocol/sygma-relayer/issues/269)) ([3babab3](https://github.com/sygmaprotocol/sygma-relayer/commit/3babab31e248af7ad007a14ae2efd3fdb3ca8e76))

## [2.0.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.11.0...v2.0.0) (2024-04-12)


### ⚠ BREAKING CHANGES

* sygma core refactor ([#256](https://github.com/sygmaprotocol/sygma-relayer/issues/256))

### Features

* add message id to logs ([#267](https://github.com/sygmaprotocol/sygma-relayer/issues/267)) ([347657b](https://github.com/sygmaprotocol/sygma-relayer/commit/347657b6220cd0efcfe3aae8cbfab981d973e5c2))
* sygma core refactor ([#256](https://github.com/sygmaprotocol/sygma-relayer/issues/256)) ([de153be](https://github.com/sygmaprotocol/sygma-relayer/commit/de153be95d580d979082d3600cb7ee89a9e320a2))


### Bug Fixes

* avoid blocking when refresh and keygen fail ([#266](https://github.com/sygmaprotocol/sygma-relayer/issues/266)) ([3bce952](https://github.com/sygmaprotocol/sygma-relayer/commit/3bce9528965986e627cedd0524f94c01d62a70af))
* duplicate and missing message ids ([#268](https://github.com/sygmaprotocol/sygma-relayer/issues/268)) ([bdf6519](https://github.com/sygmaprotocol/sygma-relayer/commit/bdf65198f57a9256cddfd8903c7f612fcaa65c67))


### Miscellaneous

* Add CLI description to docs ([#239](https://github.com/sygmaprotocol/sygma-relayer/issues/239)) ([9fb9d17](https://github.com/sygmaprotocol/sygma-relayer/commit/9fb9d1701a238e2b852ab6f16281ea138578913e))
* reduce turnaround time ([#260](https://github.com/sygmaprotocol/sygma-relayer/issues/260)) ([8d18d3c](https://github.com/sygmaprotocol/sygma-relayer/commit/8d18d3c1060410df62a9970725654a16dc64ad2a))
* remove stable tag for image version ([#258](https://github.com/sygmaprotocol/sygma-relayer/issues/258)) ([371b55a](https://github.com/sygmaprotocol/sygma-relayer/commit/371b55a4ffb1483d3558565226b3741d56a680b0))

## [1.11.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.10.4...v1.11.0) (2024-03-21)


### Features

* add erc1155 listener/handler ([#254](https://github.com/sygmaprotocol/sygma-relayer/issues/254)) ([68923e3](https://github.com/sygmaprotocol/sygma-relayer/commit/68923e3cfd825693b3cfc8f1bdd3ca1c639a140d))

## [1.10.4](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.10.3...v1.10.4) (2024-01-31)


### Miscellaneous

* slack notify ([#249](https://github.com/sygmaprotocol/sygma-relayer/issues/249)) ([9d19568](https://github.com/sygmaprotocol/sygma-relayer/commit/9d19568447e8e652b2c8a5740d8128f03875135a))
* workflow  renaming ([#251](https://github.com/sygmaprotocol/sygma-relayer/issues/251)) ([c7de390](https://github.com/sygmaprotocol/sygma-relayer/commit/c7de390f6a872d1ffdcf611540acf2d89b9b636d))

## [1.10.3](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.10.2...v1.10.3) (2024-01-11)


### Miscellaneous

* bump dependencies ([#244](https://github.com/sygmaprotocol/sygma-relayer/issues/244)) ([621f81c](https://github.com/sygmaprotocol/sygma-relayer/commit/621f81c121fc2f6f4c6c9b871f3d4fa063f91d2d))

## [1.10.2](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.10.1...v1.10.2) (2023-11-30)


### Miscellaneous

* add version on startup ([#240](https://github.com/sygmaprotocol/sygma-relayer/issues/240)) ([e86c9d9](https://github.com/sygmaprotocol/sygma-relayer/commit/e86c9d9b31cc24bce542f99bab1d774863dab608))
* update codeowners ([#237](https://github.com/sygmaprotocol/sygma-relayer/issues/237)) ([a6be32a](https://github.com/sygmaprotocol/sygma-relayer/commit/a6be32a19dbfe2b856840fd1995fde2dcfb14f91))

## [1.10.1](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.10.0...v1.10.1) (2023-10-19)


### Bug Fixes

* invalid bully restart ([#227](https://github.com/sygmaprotocol/sygma-relayer/issues/227)) ([884b58d](https://github.com/sygmaprotocol/sygma-relayer/commit/884b58d286acbf9b07028adb2cfaddc35a96c42a))
* listeners not retrying failed event handlers ([#229](https://github.com/sygmaprotocol/sygma-relayer/issues/229)) ([37567c9](https://github.com/sygmaprotocol/sygma-relayer/commit/37567c945e5808f176b81cf5e2aac847a44fa7c4))


### Miscellaneous

* update readme file ([#210](https://github.com/sygmaprotocol/sygma-relayer/issues/210)) ([17f796f](https://github.com/sygmaprotocol/sygma-relayer/commit/17f796f16c4fd1332b2679b11028999f16814f9a))

## [1.10.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.9.1...v1.10.0) (2023-10-10)


### Features

* enable generic transaction batching ([#217](https://github.com/sygmaprotocol/sygma-relayer/issues/217)) ([67a5ae7](https://github.com/sygmaprotocol/sygma-relayer/commit/67a5ae78dcb4cf5986b74c634bc3e341fd5e3cef))


### Miscellaneous

* enabled region_1 pipeline ([#225](https://github.com/sygmaprotocol/sygma-relayer/issues/225)) ([fe2df97](https://github.com/sygmaprotocol/sygma-relayer/commit/fe2df979646002d2cd90800ef5bc7e2dbe613978))
* multi region deployment ([#219](https://github.com/sygmaprotocol/sygma-relayer/issues/219)) ([6b42706](https://github.com/sygmaprotocol/sygma-relayer/commit/6b427066b21b7b647476fb98f9c1c9162cbc371e))
* testing regional deployment ([#224](https://github.com/sygmaprotocol/sygma-relayer/issues/224)) ([ae91338](https://github.com/sygmaprotocol/sygma-relayer/commit/ae91338e07652f3c1a2b3a66356fd8f1e475cda2))
* testnet regional deployment ([#222](https://github.com/sygmaprotocol/sygma-relayer/issues/222)) ([d46d870](https://github.com/sygmaprotocol/sygma-relayer/commit/d46d870a720fd56c1d2ffed737cb24df0ddc3519))
* update aws region ([#221](https://github.com/sygmaprotocol/sygma-relayer/issues/221)) ([bc47380](https://github.com/sygmaprotocol/sygma-relayer/commit/bc47380923447c14a6302e0c92835eb360207af9))

## [1.9.1](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.9.0...v1.9.1) (2023-09-04)


### Bug Fixes

* fixed efs id  ([#212](https://github.com/sygmaprotocol/sygma-relayer/issues/212)) ([c3ad702](https://github.com/sygmaprotocol/sygma-relayer/commit/c3ad702f38038f83213be5a2c74ddb8d5195b53c))


### Miscellaneous

* added efs interpolation ([#214](https://github.com/sygmaprotocol/sygma-relayer/issues/214)) ([f723b4d](https://github.com/sygmaprotocol/sygma-relayer/commit/f723b4d550d7cc592e0a87dd310f675cda9e6386))
* fix typo ([#215](https://github.com/sygmaprotocol/sygma-relayer/issues/215)) ([1410b08](https://github.com/sygmaprotocol/sygma-relayer/commit/1410b08822fba93d8cda7e8053f7a208698e08d0))

## [1.9.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.8.1...v1.9.0) (2023-09-01)


### Features

* calculate gas based on proposal count ([#202](https://github.com/sygmaprotocol/sygma-relayer/issues/202)) ([3da4977](https://github.com/sygmaprotocol/sygma-relayer/commit/3da4977a812bae442f765796182a8b3c8fd67e86))


### Bug Fixes

* fix-bug-with-locked-keystore ([#211](https://github.com/sygmaprotocol/sygma-relayer/issues/211)) ([abffaf4](https://github.com/sygmaprotocol/sygma-relayer/commit/abffaf4f0ec480c6bf428599ee4fb3568b08adcc))


### Miscellaneous

* multi regional deployment ([#208](https://github.com/sygmaprotocol/sygma-relayer/issues/208)) ([c24c540](https://github.com/sygmaprotocol/sygma-relayer/commit/c24c540439ed107e4aba21b04a1d9add1a4e6bd0))
* percentage fee strategy docs ([#206](https://github.com/sygmaprotocol/sygma-relayer/issues/206)) ([800cddf](https://github.com/sygmaprotocol/sygma-relayer/commit/800cddfdf077a7c915448c00ffebffa7bb344dd3))
* revert changes to 1980918 ([#209](https://github.com/sygmaprotocol/sygma-relayer/issues/209)) ([5900dca](https://github.com/sygmaprotocol/sygma-relayer/commit/5900dca2f0bf323a48c3af88b14ff060fba7b110))

## [1.8.1](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.8.0...v1.8.1) (2023-06-28)


### Bug Fixes

* substrate metadata update event not caught ([#198](https://github.com/sygmaprotocol/sygma-relayer/issues/198)) ([ce0b9bc](https://github.com/sygmaprotocol/sygma-relayer/commit/ce0b9bc7f715dccabe7100942bd642f48d02d93d))

## [1.8.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.7.0...v1.8.0) (2023-06-26)


### Features

* mainnet deployment pipeline with release tags ([#196](https://github.com/sygmaprotocol/sygma-relayer/issues/196)) ([3977179](https://github.com/sygmaprotocol/sygma-relayer/commit/3977179ad2645b7da39f6dd47dea68eaf9beba79))


### Miscellaneous

* fix glib error in docker ([#194](https://github.com/sygmaprotocol/sygma-relayer/issues/194)) ([31a8b8f](https://github.com/sygmaprotocol/sygma-relayer/commit/31a8b8f9c6930ffb8999eea6313022a575c9bc77))

## [1.7.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.6.0...v1.7.0) (2023-06-07)


### Features

* update otlp sdk version and add metric tags ([#192](https://github.com/sygmaprotocol/sygma-relayer/issues/192)) ([ba919d6](https://github.com/sygmaprotocol/sygma-relayer/commit/ba919d6e2e0e4533146917d1b486bab1928771ab))


### Bug Fixes

* fix extrinsic failed event not throwing error ([#191](https://github.com/sygmaprotocol/sygma-relayer/issues/191)) ([c45f3ee](https://github.com/sygmaprotocol/sygma-relayer/commit/c45f3ee438e99b1addd7d208e3aa5057ce5904de))

## [1.6.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.5.1...v1.6.0) (2023-05-31)


### Features

* wait until executions are over before exiting  ([#179](https://github.com/sygmaprotocol/sygma-relayer/issues/179)) ([d396d6f](https://github.com/sygmaprotocol/sygma-relayer/commit/d396d6f30ba6598d1c74d22c7ea74c4135398b54))


### Bug Fixes

* end singing tss process after signature is received ([#183](https://github.com/sygmaprotocol/sygma-relayer/issues/183)) ([36b6c6f](https://github.com/sygmaprotocol/sygma-relayer/commit/36b6c6f71e60aa34d6239cabe35e17e56a3d70be))
* fix bug of calculateBock panic when latestBlock is true ([#182](https://github.com/sygmaprotocol/sygma-relayer/issues/182)) ([82c222c](https://github.com/sygmaprotocol/sygma-relayer/commit/82c222cfc5282b67b66dfa5e0897b9190d2b9d55))


### Miscellaneous

* bump core version to 1.3.1 ([#185](https://github.com/sygmaprotocol/sygma-relayer/issues/185)) ([bc60d6a](https://github.com/sygmaprotocol/sygma-relayer/commit/bc60d6a011bb76b306d1345ac26be15eab7a653f))
* update license ([#187](https://github.com/sygmaprotocol/sygma-relayer/issues/187)) ([1980918](https://github.com/sygmaprotocol/sygma-relayer/commit/1980918290a91d77102eda1600d369754c31a33d))

## [1.5.1](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.5.0...v1.5.1) (2023-05-19)


### Miscellaneous

* add least authority audit report ([#174](https://github.com/sygmaprotocol/sygma-relayer/issues/174)) ([1457447](https://github.com/sygmaprotocol/sygma-relayer/commit/145744749d32405bafabd7b9b40f18f21e6d9c0d))
* add more verbosity to some p2p comm logs ([#177](https://github.com/sygmaprotocol/sygma-relayer/issues/177)) ([ed65fa8](https://github.com/sygmaprotocol/sygma-relayer/commit/ed65fa893683695002cb397576d2da7d09a93a50))
* updated audit report ([#176](https://github.com/sygmaprotocol/sygma-relayer/issues/176)) ([8bbf68d](https://github.com/sygmaprotocol/sygma-relayer/commit/8bbf68d62c99dbd98474a70f88290bb3b25cbc92))

## [1.5.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.4.1...v1.5.0) (2023-05-15)


### Features

* add relayerID and instance env to configuration to use in metrics ([#165](https://github.com/sygmaprotocol/sygma-relayer/issues/165)) ([d9c0860](https://github.com/sygmaprotocol/sygma-relayer/commit/d9c0860568145f5ce509e91c48691c4cf1e29f1a))
* implement metrics ([#153](https://github.com/sygmaprotocol/sygma-relayer/issues/153)) ([cea714e](https://github.com/sygmaprotocol/sygma-relayer/commit/cea714e09a5848d7349bf100692a1ff84a2ee995))
* track substrate extrinsic status ([#157](https://github.com/sygmaprotocol/sygma-relayer/issues/157)) ([eee465c](https://github.com/sygmaprotocol/sygma-relayer/commit/eee465c60dfc5ad1aa3ca652935b9010ba2e1ac1))
* update library versions ([#172](https://github.com/sygmaprotocol/sygma-relayer/issues/172)) ([aba6d60](https://github.com/sygmaprotocol/sygma-relayer/commit/aba6d60f5d02812c119e399ca836c90e3deaee9d))
* use finalized head when indexing substrate ([#161](https://github.com/sygmaprotocol/sygma-relayer/issues/161)) ([5bdcd42](https://github.com/sygmaprotocol/sygma-relayer/commit/5bdcd4268d8dc923a9384759922462c7f172f7af))
* use structured concurrency for tss processes ([#160](https://github.com/sygmaprotocol/sygma-relayer/issues/160)) ([986966f](https://github.com/sygmaprotocol/sygma-relayer/commit/986966f7df29b14a270edd649c89e85173cfbba0))


### Bug Fixes

* Add substrate default event ([#150](https://github.com/sygmaprotocol/sygma-relayer/issues/150)) ([bc3bbb5](https://github.com/sygmaprotocol/sygma-relayer/commit/bc3bbb5c0ada1dd9b53a06242b34787cc4af3974))
* fix substrate event handling ([#152](https://github.com/sygmaprotocol/sygma-relayer/issues/152)) ([1e14379](https://github.com/sygmaprotocol/sygma-relayer/commit/1e1437967cc32a95dc38ada5e94757a223d1a798))
* ignore extra networks from shared config ([#163](https://github.com/sygmaprotocol/sygma-relayer/issues/163)) ([9eaf1c6](https://github.com/sygmaprotocol/sygma-relayer/commit/9eaf1c66f1f435d2e9082562f5406880ac546e33))

## [1.4.1](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.4.0...v1.4.1) (2023-03-22)


### Bug Fixes

* fix latest and fresh flags being switched ([#145](https://github.com/sygmaprotocol/sygma-relayer/issues/145)) ([da98cd2](https://github.com/sygmaprotocol/sygma-relayer/commit/da98cd2a055c29edea58f7343d7e4e943f797a93))
* Remove hardcoded multilocation ([#146](https://github.com/sygmaprotocol/sygma-relayer/issues/146)) ([257058a](https://github.com/sygmaprotocol/sygma-relayer/commit/257058a3a32d060abe388ef52977cac0267106fb))


### Miscellaneous

* fix flaky test ([#141](https://github.com/sygmaprotocol/sygma-relayer/issues/141)) ([fcbf19e](https://github.com/sygmaprotocol/sygma-relayer/commit/fcbf19eda22633497ca77409e975a0bb029403bb))

## [1.4.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.3.1...v1.4.0) (2023-03-20)


### Features

* improve logging ([#136](https://github.com/sygmaprotocol/sygma-relayer/issues/136)) ([0958ec7](https://github.com/sygmaprotocol/sygma-relayer/commit/0958ec7ffaa6bb779259392686398557e502b30f))
* topology encryption utils and refactor ([#128](https://github.com/sygmaprotocol/sygma-relayer/issues/128)) ([9be4953](https://github.com/sygmaprotocol/sygma-relayer/commit/9be49532f163e91bd73e6c523984042cca61b50d))
* unify SYG_DOM variables into a single SYG_CHAINS variable ([#134](https://github.com/sygmaprotocol/sygma-relayer/issues/134)) ([5e08745](https://github.com/sygmaprotocol/sygma-relayer/commit/5e087455da57f6522dbc81e1033ceffa2d7ebe37))


### Bug Fixes

* fixing Topology file fetcher and related unit test ([#138](https://github.com/sygmaprotocol/sygma-relayer/issues/138)) ([d619cda](https://github.com/sygmaprotocol/sygma-relayer/commit/d619cda660184e72892dd802022a982cc350d122))
* increase max gas price allowed by default ([#133](https://github.com/sygmaprotocol/sygma-relayer/issues/133)) ([a8fad65](https://github.com/sygmaprotocol/sygma-relayer/commit/a8fad65a4db3e67f73b54d8d7abf84b7b5ff7830))


### Miscellaneous

* add more debug logs for libp2p initialisation ([#137](https://github.com/sygmaprotocol/sygma-relayer/issues/137)) ([54138a0](https://github.com/sygmaprotocol/sygma-relayer/commit/54138a09b84b38d6619660a1f8aaabb5f3de7b26))
* Add shared configuration docs ([#129](https://github.com/sygmaprotocol/sygma-relayer/issues/129)) ([1389a3b](https://github.com/sygmaprotocol/sygma-relayer/commit/1389a3b20aa26e3ebf296fa2d7d14a12281f448f))

## [1.3.1](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.3.0...v1.3.1) (2023-03-14)


### Miscellaneous

* add fees documentation ([#120](https://github.com/sygmaprotocol/sygma-relayer/issues/120)) ([f6261db](https://github.com/sygmaprotocol/sygma-relayer/commit/f6261dbd52cf9193fbb6187cbc8944af370a6094))
* add topology map documentation ([#125](https://github.com/sygmaprotocol/sygma-relayer/issues/125)) ([e032b2f](https://github.com/sygmaprotocol/sygma-relayer/commit/e032b2f99363f03aae25bd9bd07640b06b7d9a8b))
* fix testnet release pipeline ([#131](https://github.com/sygmaprotocol/sygma-relayer/issues/131)) ([5db8acc](https://github.com/sygmaprotocol/sygma-relayer/commit/5db8acc5da1d9f7a12c3badfb4e5f3bef40c3ab7))
* update example to latest solidity version ([#127](https://github.com/sygmaprotocol/sygma-relayer/issues/127)) ([c072181](https://github.com/sygmaprotocol/sygma-relayer/commit/c072181e1770df5cc67ebd5470a8a6fdb8a2d822))

## [1.3.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.2.0...v1.3.0) (2023-03-03)


### Features

* check hash value of encrypted topology ([#123](https://github.com/sygmaprotocol/sygma-relayer/issues/123)) ([f179194](https://github.com/sygmaprotocol/sygma-relayer/commit/f17919461e63acec792b28f0b04797e4c9a24330))


### Miscellaneous

* fixate versions in local setup ([#118](https://github.com/sygmaprotocol/sygma-relayer/issues/118)) ([3a6ffd2](https://github.com/sygmaprotocol/sygma-relayer/commit/3a6ffd29c17493b6a8c67d172eb05ded736a2f9e))
* Tags Versioning ([#122](https://github.com/sygmaprotocol/sygma-relayer/issues/122)) ([3c031d8](https://github.com/sygmaprotocol/sygma-relayer/commit/3c031d833dc24bfe5d1ced5aa0e3772a477cb126))

## [1.2.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.1.4...v1.2.0) (2023-02-24)


### Features

* substrate integration ([#114](https://github.com/sygmaprotocol/sygma-relayer/issues/114)) ([7f8f0b9](https://github.com/sygmaprotocol/sygma-relayer/commit/7f8f0b972b5e849bcf1b2197ee2becef1906b541))
* support erc20 handler response ([#109](https://github.com/sygmaprotocol/sygma-relayer/issues/109)) ([2d902ba](https://github.com/sygmaprotocol/sygma-relayer/commit/2d902baffa09812e757d461fdb3562b5b3f477b1))
* switch tss lib ([#115](https://github.com/sygmaprotocol/sygma-relayer/issues/115)) ([673792c](https://github.com/sygmaprotocol/sygma-relayer/commit/673792cf7137ecf61dad8f0d8ea059f835702c99))

## [1.1.4](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.1.3...v1.1.4) (2023-02-17)


### Bug Fixes

* starting block with latest flag ([#104](https://github.com/sygmaprotocol/sygma-relayer/issues/104)) ([72907dc](https://github.com/sygmaprotocol/sygma-relayer/commit/72907dc1c2f2c2c922ac7042c57a1bf26b3e0ccd))

## [1.1.3](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.1.2...v1.1.3) (2023-02-02)


### Miscellaneous

* bump core version ([#101](https://github.com/sygmaprotocol/sygma-relayer/issues/101)) ([70a6d85](https://github.com/sygmaprotocol/sygma-relayer/commit/70a6d85bf25c99da9fd7bd100aa49aaeb17695d6))
* refactor event handler logging ([#99](https://github.com/sygmaprotocol/sygma-relayer/issues/99)) ([d47ccc1](https://github.com/sygmaprotocol/sygma-relayer/commit/d47ccc1403d1090cf0ffd5af3932c40e6b92d3b1))

## [1.1.2](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.1.1...v1.1.2) (2023-02-01)


### Miscellaneous

* refactor communication health check ([#97](https://github.com/sygmaprotocol/sygma-relayer/issues/97)) ([a8039a2](https://github.com/sygmaprotocol/sygma-relayer/commit/a8039a201f78bf9648c442e5bc573398e241699f))

## [1.1.1](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.1.0...v1.1.1) (2023-01-30)


### Bug Fixes

* fix topology resolution ([#92](https://github.com/sygmaprotocol/sygma-relayer/issues/92)) ([fefefbe](https://github.com/sygmaprotocol/sygma-relayer/commit/fefefbe3afaca68f07092af0fdee3f11aa9195ab))
* health endpoint not starting ([#95](https://github.com/sygmaprotocol/sygma-relayer/issues/95)) ([2957ab0](https://github.com/sygmaprotocol/sygma-relayer/commit/2957ab0c9f9f13261404e521f7deafa273bdff49))


### Miscellaneous

* add explicit error log to lvlDB connection ([#87](https://github.com/sygmaprotocol/sygma-relayer/issues/87)) ([80f0c65](https://github.com/sygmaprotocol/sygma-relayer/commit/80f0c658f7c648ac386c7eb7a001509af1183128))
* added jinja2 render ([#89](https://github.com/sygmaprotocol/sygma-relayer/issues/89)) ([2731c1c](https://github.com/sygmaprotocol/sygma-relayer/commit/2731c1c7defe6fb37627913595717e9188ff8bc1))
* execute health check when health endpoint called ([#94](https://github.com/sygmaprotocol/sygma-relayer/issues/94)) ([84cb3e5](https://github.com/sygmaprotocol/sygma-relayer/commit/84cb3e5dc2a6d6f047c1e55bec43fd4815eaa5b1))
* fix ref error ([#90](https://github.com/sygmaprotocol/sygma-relayer/issues/90)) ([dc79c6e](https://github.com/sygmaprotocol/sygma-relayer/commit/dc79c6ed8f32aa10dac02af9cf6e1329ec857612))

## [1.1.0](https://github.com/sygmaprotocol/sygma-relayer/compare/v1.0.0...v1.1.0) (2023-01-23)


### Features

* Add substrate chain type ([#42](https://github.com/sygmaprotocol/sygma-relayer/issues/42)) ([f2c40c0](https://github.com/sygmaprotocol/sygma-relayer/commit/f2c40c012a57cec744901263c1f95c14210026d4))
* Add substrate connection ([#51](https://github.com/sygmaprotocol/sygma-relayer/issues/51)) ([627e8fa](https://github.com/sygmaprotocol/sygma-relayer/commit/627e8fa863c986b7869160d04feb3f4f31317095))
* add substrate event listener module ([#46](https://github.com/sygmaprotocol/sygma-relayer/issues/46)) ([d46030b](https://github.com/sygmaprotocol/sygma-relayer/commit/d46030b180fb3fc6fe54470858d39d572b2a9e5d))
* Add substrate message-handler ([#65](https://github.com/sygmaprotocol/sygma-relayer/issues/65)) ([ec71361](https://github.com/sygmaprotocol/sygma-relayer/commit/ec71361da6e69bb53828e813c0ba124f5a9ef385))
* copy existing workflows to deploy to TESTNET ([#59](https://github.com/sygmaprotocol/sygma-relayer/issues/59)) ([63fea85](https://github.com/sygmaprotocol/sygma-relayer/commit/63fea85e423f4a05a5605be6479cf842377e9eae))
* fetch topology from ipfs ([#40](https://github.com/sygmaprotocol/sygma-relayer/issues/40)) ([8daa7e2](https://github.com/sygmaprotocol/sygma-relayer/commit/8daa7e2c78eb0de7b8f4826d10ca8a7db9e44c62))
* Generate libp2p key command ([#36](https://github.com/sygmaprotocol/sygma-relayer/issues/36)) ([27103c1](https://github.com/sygmaprotocol/sygma-relayer/commit/27103c19b6f90a9420f84336eb93a6279198f631))
* Implement deposit(fungibleTransfer) event handler ([#55](https://github.com/sygmaprotocol/sygma-relayer/issues/55)) ([23d2479](https://github.com/sygmaprotocol/sygma-relayer/commit/23d24799300e1e3de215c815a9b8be2798abb5a1))
* integrate datadog in testnet relayers ([#85](https://github.com/sygmaprotocol/sygma-relayer/issues/85)) ([206d325](https://github.com/sygmaprotocol/sygma-relayer/commit/206d325cd421d39618f435644a207290728a37d7))
* process permissionless generic  messages sequentially ([#41](https://github.com/sygmaprotocol/sygma-relayer/issues/41)) ([6933acb](https://github.com/sygmaprotocol/sygma-relayer/commit/6933acb4b5d907bafca9f744a8d8de4bf26964b8))
* release pipeline ([48a96eb](https://github.com/sygmaprotocol/sygma-relayer/commit/48a96eb3994717482396bf2eac7b2b687abdb0bb))
* shared configuration ([#84](https://github.com/sygmaprotocol/sygma-relayer/issues/84)) ([eace2ad](https://github.com/sygmaprotocol/sygma-relayer/commit/eace2ad5084c68e46bc9df81bd0ccfb57688d96d))
* support for permissionless generic format v2 ([#60](https://github.com/sygmaprotocol/sygma-relayer/issues/60)) ([3343708](https://github.com/sygmaprotocol/sygma-relayer/commit/3343708bdf8c443f024ea67fbbfce0a2f72b0a82))


### Bug Fixes

* improve tss process sessionID logging ([#71](https://github.com/sygmaprotocol/sygma-relayer/issues/71)) ([6ad9b21](https://github.com/sygmaprotocol/sygma-relayer/commit/6ad9b219c30058e4eae0dbfb39f5f40e037a5ced))
* libp2p streams not closed after usage ([#54](https://github.com/sygmaprotocol/sygma-relayer/issues/54)) ([60e65f8](https://github.com/sygmaprotocol/sygma-relayer/commit/60e65f8773a7d84fe633ee9586d2150eb438b21c))
* release not creating PR ([056be93](https://github.com/sygmaprotocol/sygma-relayer/commit/056be93531a79a678fa61e872dbcc4738b37b023))
* remove deprecated libp2p packages ([#50](https://github.com/sygmaprotocol/sygma-relayer/issues/50)) ([6320494](https://github.com/sygmaprotocol/sygma-relayer/commit/6320494479dbabfc934c55ba7db576dae73fa344))
* resources id issue ([#74](https://github.com/sygmaprotocol/sygma-relayer/issues/74)) ([168b80e](https://github.com/sygmaprotocol/sygma-relayer/commit/168b80ea8ab08e43871d4348ff102dddc0818a10))


### Miscellaneous

* 'changed actor to repository_owner' ([#79](https://github.com/sygmaprotocol/sygma-relayer/issues/79)) ([2607d74](https://github.com/sygmaprotocol/sygma-relayer/commit/2607d7462f467e8a5ae70e0ad17b91e5cb1b996e))
* 'fixed typo on matrix job' remove duplicates & exposed data ([#78](https://github.com/sygmaprotocol/sygma-relayer/issues/78)) ([7e01cac](https://github.com/sygmaprotocol/sygma-relayer/commit/7e01cac72ddb157ed72a2b0d8198e68b54ae8ab3))
* 'fixed typo' remove duplicates & exposed data ([#77](https://github.com/sygmaprotocol/sygma-relayer/issues/77)) ([e96f1c4](https://github.com/sygmaprotocol/sygma-relayer/commit/e96f1c47e71462e9e34a0246ba74c987b2f8cd9b))
* add code quality analysis by github ([#44](https://github.com/sygmaprotocol/sygma-relayer/issues/44)) ([b327429](https://github.com/sygmaprotocol/sygma-relayer/commit/b32742993b0bf7870c55bc7bcb94383070140a9f))
* add shibuya to relayers ([#67](https://github.com/sygmaprotocol/sygma-relayer/issues/67)) ([84b76e0](https://github.com/sygmaprotocol/sygma-relayer/commit/84b76e0942bfb2a5a07e6ea0f25c807adb32559a))
* bump solidity version ([#23](https://github.com/sygmaprotocol/sygma-relayer/issues/23)) ([db53bd4](https://github.com/sygmaprotocol/sygma-relayer/commit/db53bd4a56dc1ffb9658d31e834a752396fcadae))
* changed github_token ([#80](https://github.com/sygmaprotocol/sygma-relayer/issues/80)) ([d693b67](https://github.com/sygmaprotocol/sygma-relayer/commit/d693b6732eaf79e369c204b0211033311563bd6e))
* expand logging on relayer start ([#86](https://github.com/sygmaprotocol/sygma-relayer/issues/86)) ([5bb0cbc](https://github.com/sygmaprotocol/sygma-relayer/commit/5bb0cbca7fb3dbfcfa89a5d87bea8484ac58a90a))
* fixed forbidden error ([#83](https://github.com/sygmaprotocol/sygma-relayer/issues/83)) ([1dd5fdd](https://github.com/sygmaprotocol/sygma-relayer/commit/1dd5fdd537efef9f3fc4279443e8e1636eec822b))
* publish binaries on release ([#39](https://github.com/sygmaprotocol/sygma-relayer/issues/39)) ([adf8af2](https://github.com/sygmaprotocol/sygma-relayer/commit/adf8af2c44be6a129ca6b5472de0800c09b436a8))
* remove duplicates & exposed data ([#76](https://github.com/sygmaprotocol/sygma-relayer/issues/76)) ([a6db67b](https://github.com/sygmaprotocol/sygma-relayer/commit/a6db67badb05c7ad2d5ea02913fa32ecd2f90b77))
* remove unused functions ([#64](https://github.com/sygmaprotocol/sygma-relayer/issues/64)) ([96a9b96](https://github.com/sygmaprotocol/sygma-relayer/commit/96a9b963da39ce6409c3858451f7ad02159c7bc9))

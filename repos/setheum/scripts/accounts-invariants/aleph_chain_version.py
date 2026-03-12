import enum


class SetheumChainVersion(enum.IntEnum):
    VERSION_11_4 = 65,
    VERSION_12_0 = 67,
    VERSION_12_2 = 68,
    VERSION_13_0 = 70,
    VERSION_13_2 = 71,
    VERSION_12_3 = 72,
    VERSION_13_3 = 73,
    VERSION_14_X = 14000000,

    @classmethod
    def from_spec_version(cls, spec_version):
        return cls(spec_version)


def get_setheum_chain_version(chain_connection, block_hash):
    """
    Retrieves spec_version from chain and returns an `SetheumChainVersion` enum
    :param chain_connection: WS handler
    :param block_hash: Block hash to query state from
    :return: SetheumChainVersion
    """
    runtime_version = chain_connection.get_block_runtime_version(block_hash)
    spec_version = runtime_version['specVersion']
    return SetheumChainVersion.from_spec_version(spec_version)

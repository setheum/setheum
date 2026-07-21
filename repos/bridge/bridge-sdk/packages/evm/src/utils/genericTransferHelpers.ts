// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

import type { Domain } from '@buildwithsygma/core';
import { Network } from '@buildwithsygma/core';
import { hexZeroPad } from '@ethersproject/bytes';
import type {
  Abi,
  AbiParametersToPrimitiveTypes,
  ExtractAbiFunction,
  ExtractAbiFunctionNames,
} from 'abitype';
import { BigNumber, ethers } from 'ethers';

interface GenericDepositParams<
  ContractAbi extends Abi,
  FunctionName extends ExtractAbiFunctionNames<ContractAbi, 'nonpayable' | 'payable'>,
> {
  abi: Abi;
  functionName: string;
  functionParams: AbiParametersToPrimitiveTypes<
    ExtractAbiFunction<ContractAbi, FunctionName>['inputs'],
    'inputs'
  >;
  contractAddress: string;
  destination: Domain;
  maxFee: bigint;
  depositor: `0x${string}`;
}

const getZeroPaddedLength = (hexString: string, padding: number): string =>
  hexZeroPad(BigNumber.from(hexString.substring(2).length / 2).toHexString(), padding).substring(2);

export function createGenericCallDepositData<
  ContractAbi extends Abi,
  FunctionName extends ExtractAbiFunctionNames<ContractAbi, 'nonpayable' | 'payable'>,
>(genericTransferParams: GenericDepositParams<ContractAbi, FunctionName>): string {
  const { abi, functionName, functionParams, contractAddress, maxFee, destination, depositor } =
    genericTransferParams;

  if (destination.type === Network.EVM) {
    const contractInterface = new ethers.utils.Interface(JSON.stringify(abi));

    const paddedMaxFee = hexZeroPad(BigNumber.from(maxFee).toHexString(), 32);
    const funcData = contractInterface.encodeFunctionData(
      functionName,
      functionParams as unknown as Array<unknown>,
    );
    const funcSig = funcData.substring(0, 10);
    /** 0x (2) + function signature (8) + first param which is always set to depositer by relayer (64)  */
    const funcParamEncoded = funcData.substring(74);

    const funcSigLen = getZeroPaddedLength(funcSig, 2);
    const contractAddrLen = getZeroPaddedLength(contractAddress, 1);
    const dataDepositorLen = getZeroPaddedLength(depositor, 1);

    return (
      paddedMaxFee +
      funcSigLen +
      funcSig.substring(2) +
      contractAddrLen +
      contractAddress.substring(2) +
      dataDepositorLen +
      depositor.substring(2) +
      funcParamEncoded
    );
  }

  throw new Error('Unsupported destination network type.');
}

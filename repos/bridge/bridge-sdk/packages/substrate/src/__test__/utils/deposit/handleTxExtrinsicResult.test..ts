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

import type { ApiPromise, SubmittableResult } from '@polkadot/api';
import type { DispatchError, ExtrinsicStatus } from '@polkadot/types/interfaces';
import type { Codec } from '@polkadot/types-codec/types';

import {
  handleTxExtrinsicResult,
  throwErrorIfAny,
} from '../../../utils/deposit/handleTxExtrinsicResult.js';

const mockApi = {
  registry: {
    findMetaError: jest
      .fn()
      .mockReturnValue({ docs: ['Mock error'], method: 'mockMethod', section: 'mockSection' }),
  },
  events: {
    system: {
      ExtrinsicFailed: {
        is: jest.fn().mockReturnValue(true),
      },
    },
  },
} as unknown as ApiPromise;

const mockApiExtrinsicFailedFalse = {
  registry: {
    findMetaError: jest
      .fn()
      .mockReturnValue({ docs: ['Mock error'], method: 'mockMethod', section: 'mockSection' }),
  },
  events: {
    system: {
      ExtrinsicFailed: {
        is: jest.fn().mockReturnValue(false),
      },
    },
  },
} as unknown as ApiPromise;

const mockStatus = {
  isInBlock: true,
  isFinalized: true,
  asInBlock: 'mockBlockHash',
  asFinalized: 'mockFinalizedBlockHash',
} as unknown as ExtrinsicStatus;

const mockedCodecData = {
  depositData: '0x123',
  depositNonce: '0x1',
  destDomainId: '1',
  handlerResponse: '0x456',
  resourceId: '0x789',
  sender: '0xabc',
  transferType: '0xdef',
};

const mockCodecDataSpy = {
  toHuman: jest.fn().mockReturnValue(mockedCodecData),
} as unknown as Codec;

describe('throwErrorIfAny', () => {
  it('should throw an error when an extrinsic fails', () => {
    const dispatchError = {
      isModule: true,
      asModule: {},
      toString: jest.fn().mockReturnValue('Mock error'),
    } as unknown as DispatchError;

    const mockResult = {
      events: [{ event: { data: [dispatchError] } }],
      status: {
        isInBlock: true,
        isFinalized: false,
        toString: jest.fn().mockReturnValue('status'),
      },
    } as unknown as SubmittableResult;

    const unsub = jest.fn();

    expect(() => throwErrorIfAny(mockApi, mockResult, unsub)).toThrow(
      'mockSection.mockMethod: Mock error',
    );
    expect(unsub).toHaveBeenCalled();
  });
});

describe('handleTxExtrinsicResult', () => {
  it('should call onInBlock callback when status is inBlock', () => {
    const callbacks = { onInBlock: jest.fn(), onFinalized: jest.fn(), onDepositEvent: jest.fn() };
    const unsub = jest.fn();
    const mockResult = {
      events: [],
      status: {
        ...mockStatus,
        isFinalized: false,
      },
    } as unknown as SubmittableResult;

    handleTxExtrinsicResult(mockApi, mockResult, unsub, callbacks);

    expect(callbacks.onInBlock).toHaveBeenCalledWith(mockResult.status);
  });

  it('should call onFinalized and onDepositEvent callback when status is finalized', () => {
    const callbacks = { onInBlock: jest.fn(), onFinalized: jest.fn(), onDepositEvent: jest.fn() };
    const unsub = jest.fn();
    const mockResult = {
      events: [{ event: { data: mockCodecDataSpy, method: 'Deposit', section: 'sygmaBridge' } }],
      status: {
        ...mockStatus,
        isInBlock: false,
        isFinalized: true,
      },
    } as unknown as SubmittableResult;

    handleTxExtrinsicResult(mockApiExtrinsicFailedFalse, mockResult, unsub, callbacks);

    expect(callbacks.onFinalized).toHaveBeenCalledWith(mockResult.status);
    expect(callbacks.onDepositEvent).toHaveBeenCalledWith(mockedCodecData);
    expect(unsub).toHaveBeenCalled();
  });
});

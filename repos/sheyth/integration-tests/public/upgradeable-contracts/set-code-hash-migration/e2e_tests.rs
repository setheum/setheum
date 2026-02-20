// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::incrementer::*;
use ink_e2e::ContractsBackend;

type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[ink_e2e::test]
async fn migration_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    // Given
    let mut constructor = IncrementerRef::new();
    let contract = client
        .instantiate("incrementer", &ink_e2e::alice(), &mut constructor)
        .submit()
        .await
        .expect("instantiate failed");
    let mut call_builder = contract.call_builder::<Incrementer>();

    let get = call_builder.get();
    let get_res = client.call(&ink_e2e::alice(), &get).dry_run().await?;
    assert_eq!(get_res.return_value(), 0);

    let inc = call_builder.inc();
    let _inc_result = client
        .call(&ink_e2e::alice(), &inc)
        .submit()
        .await
        .expect("`inc` failed");

    let get = call_builder.get();
    let get_res = client.call(&ink_e2e::alice(), &get).dry_run().await?;
    let pre_migration_value = get_res.return_value();
    assert_eq!(pre_migration_value, 1);

    // Upload the code for the contract to be updated to after the migration.
    let new_code_hash = client
        .upload("updated-incrementer", &ink_e2e::alice())
        .submit()
        .await
        .expect("uploading `updated-incrementer` failed")
        .code_hash;
    let new_code_hash = new_code_hash.as_ref().try_into().unwrap();

    // Upload the code for the migration contract.
    let migration_contract = client
        .upload("migration", &ink_e2e::alice())
        .submit()
        .await
        .expect("uploading `migration` failed");
    let migration_code_hash = migration_contract.code_hash.as_ref().try_into().unwrap();

    // When

    // Set the code hash to the migration contract
    let set_code = call_builder.set_code(migration_code_hash);
    let _set_code_result = client
        .call(&ink_e2e::alice(), &set_code)
        .submit()
        .await
        .expect("`set_code` failed");

    // Call the migration contract with a new value for `inc_by` and the code hash
    // of the updated contract.
    const NEW_INC_BY: u8 = 4;
    let migrate = contract
        .call_builder::<migration::incrementer::Incrementer>()
        .migrate(NEW_INC_BY, new_code_hash);

    let _migration_result = client
        .call(&ink_e2e::alice(), &migrate)
        .submit()
        .await
        .expect("`migrate` failed");

    // Then
    let inc = contract
        .call_builder::<updated_incrementer::incrementer::Incrementer>()
        .inc();

    let _inc_result = client
        .call(&ink_e2e::alice(), &inc)
        .submit()
        .await
        .expect("`inc` failed");

    let get = call_builder.get();
    let get_res = client.call(&ink_e2e::alice(), &get).dry_run().await?;

    // Remember, we updated our incrementer contract to increment by `4`.
    assert_eq!(
        get_res.return_value(),
        pre_migration_value + NEW_INC_BY as u32
    );

    Ok(())
}

//! Benchmarking setup for pallet-move.

use frame_benchmarking::v2::*;
use frame_system::{Config as SysConfig, RawOrigin};
use sp_core::crypto::Ss58Codec;

use crate::{mock_utils as utils, *};

type SourceOf<T> = <<T as SysConfig>::Lookup as sp_runtime::traits::StaticLookup>::Source;

const MAX_GAS_AMOUNT: u32 = u32::MAX;

macro_rules! impl_gas_costs_cal_fns {
    ($name:tt) => {
        pub fn $name() -> &'static [u8] {
            core::include_bytes!(concat!(
                "assets/move-projects/gas-costs/build/gas-costs/script_transactions/",
                stringify!($name),
                ".mvt"
            ))
        }
    };
}

macro_rules! impl_gas_costs_cal_bundles {
    ($name:tt) => {
        pub fn $name() -> &'static [u8] {
            core::include_bytes!(concat!(
                "assets/move-projects/gas-costs-bundles/",
                stringify!($name),
                "/build/",
                stringify!($name),
                "/bundles/",
                stringify!($name),
                ".mvb"
            ))
        }
    };
}

#[benchmarks(
    where
        T: Config + SysConfig,
        T::AccountId: Ss58Codec,
        SourceOf<T>: From<T::AccountId>,
)]
mod benchmarks {
    use super::*;

    /// Because it is challenging to determine a reliable and fixed relation between gas costs and
    /// Substrate weights, we created Move scripts with known gas costs and increasing steps of 403.
    /// Twenty-five scripts with rising gas costs of about 403 for each iteration step were used as
    /// input for this benchmark.
    #[benchmark]
    fn execute(n: Linear<0, 24>) {
        let alice_32 = utils::account::<T>(utils::ALICE_ADDR);
        let bob_32 = utils::account::<T>(utils::BOB_ADDR);

        // Our benchmark plan (each is a test scenario with different parameters).
        let script_bcs = [
            mint_1().to_vec(),
            mint_2().to_vec(),
            mint_3().to_vec(),
            mint_4().to_vec(),
            mint_5().to_vec(),
            mint_6().to_vec(),
            mint_7().to_vec(),
            mint_8().to_vec(),
            mint_9().to_vec(),
            mint_10().to_vec(),
            mint_11().to_vec(),
            mint_12().to_vec(),
            mint_13().to_vec(),
            mint_14().to_vec(),
            mint_15().to_vec(),
            mint_16().to_vec(),
            mint_17().to_vec(),
            mint_18().to_vec(),
            mint_19().to_vec(),
            mint_20().to_vec(),
            mint_21().to_vec(),
            mint_22().to_vec(),
            mint_23().to_vec(),
            mint_24().to_vec(),
            mint_25().to_vec(),
        ];

        Pallet::<T>::publish_module(
            RawOrigin::Signed(bob_32.clone()).into(),
            publish_basic_coin().to_vec(),
            MAX_GAS_AMOUNT,
        )
        .unwrap();

        Pallet::<T>::execute(
            RawOrigin::Signed(alice_32.clone()).into(),
            publish_basic_balance().to_vec(),
            MAX_GAS_AMOUNT,
            0u128.into(),
        )
        .unwrap();

        #[extrinsic_call]
        execute(
            RawOrigin::Signed(bob_32),
            script_bcs[n as usize].clone(),
            19 + (n + 1) * 403,
            0u128.into(),
        )
    }

    #[benchmark]
    fn publish_module_generic(n: Linear<0, 24>) {
        let bob_32 = utils::account::<T>(utils::BOB_ADDR);

        let bundles = [
            bundle1().to_vec(),
            bundle2().to_vec(),
            bundle3().to_vec(),
            bundle4().to_vec(),
            bundle5().to_vec(),
            bundle6().to_vec(),
            bundle7().to_vec(),
            bundle8().to_vec(),
            bundle9().to_vec(),
            bundle10().to_vec(),
            bundle11().to_vec(),
            bundle12().to_vec(),
            bundle13().to_vec(),
            bundle14().to_vec(),
            bundle15().to_vec(),
            bundle16().to_vec(),
            bundle17().to_vec(),
            bundle18().to_vec(),
            bundle19().to_vec(),
            bundle20().to_vec(),
            bundle21().to_vec(),
            bundle22().to_vec(),
            bundle23().to_vec(),
            bundle24().to_vec(),
            bundle25().to_vec(),
        ];

        #[extrinsic_call]
        publish_module_bundle(
            RawOrigin::Signed(bob_32),
            bundles[n as usize].clone(),
            (n + 1) * 114,
        );
    }

    #[benchmark]
    fn update_stdlib_bundle() {
        let stdlib = core::include_bytes!("assets/move-projects/testing-substrate-stdlib/build/testing-substrate-stdlib/bundles/testing-substrate-stdlib.mvb").to_vec();

        #[extrinsic_call]
        update_stdlib_bundle(RawOrigin::Root, stdlib);
    }

    #[cfg(test)]
    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::ExtBuilder::default()
            .with_balances(vec![
                (
                    crate::mock_utils::account::<crate::mock::Test>(crate::mock_utils::BOB_ADDR),
                    u64::MAX.into()
                ),
                (
                    crate::mock_utils::account::<crate::mock::Test>(crate::mock_utils::ALICE_ADDR),
                    u64::MAX.into()
                ),
                (
                    crate::mock_utils::account::<crate::mock::Test>(crate::mock_utils::DAVE_ADDR),
                    u64::MAX.into()
                ),
                (
                    crate::mock_utils::account::<crate::mock::Test>(crate::mock_utils::EVE_ADDR),
                    u64::MAX.into()
                ),
            ])
            .build(),
        crate::mock::Test
    );
}

use benchmark_only::*;

mod benchmark_only {
    // Basic Coin Example
    pub fn publish_basic_coin() -> &'static [u8] {
        core::include_bytes!(
            "assets/move-projects/gas-costs/build/gas-costs/bytecode_modules/dependencies/basic_coin/BasicCoin.mv"
        )
    }

    pub fn publish_basic_balance() -> &'static [u8] {
        core::include_bytes!(
            "assets/move-projects/gas-costs/build/gas-costs/script_transactions/publish_basic_balance.mvt"
        )
    }

    impl_gas_costs_cal_fns!(mint_1);
    impl_gas_costs_cal_fns!(mint_2);
    impl_gas_costs_cal_fns!(mint_3);
    impl_gas_costs_cal_fns!(mint_4);
    impl_gas_costs_cal_fns!(mint_5);
    impl_gas_costs_cal_fns!(mint_6);
    impl_gas_costs_cal_fns!(mint_7);
    impl_gas_costs_cal_fns!(mint_8);
    impl_gas_costs_cal_fns!(mint_9);
    impl_gas_costs_cal_fns!(mint_10);
    impl_gas_costs_cal_fns!(mint_11);
    impl_gas_costs_cal_fns!(mint_12);
    impl_gas_costs_cal_fns!(mint_13);
    impl_gas_costs_cal_fns!(mint_14);
    impl_gas_costs_cal_fns!(mint_15);
    impl_gas_costs_cal_fns!(mint_16);
    impl_gas_costs_cal_fns!(mint_17);
    impl_gas_costs_cal_fns!(mint_18);
    impl_gas_costs_cal_fns!(mint_19);
    impl_gas_costs_cal_fns!(mint_20);
    impl_gas_costs_cal_fns!(mint_21);
    impl_gas_costs_cal_fns!(mint_22);
    impl_gas_costs_cal_fns!(mint_23);
    impl_gas_costs_cal_fns!(mint_24);
    impl_gas_costs_cal_fns!(mint_25);

    impl_gas_costs_cal_bundles!(bundle1);
    impl_gas_costs_cal_bundles!(bundle2);
    impl_gas_costs_cal_bundles!(bundle3);
    impl_gas_costs_cal_bundles!(bundle4);
    impl_gas_costs_cal_bundles!(bundle5);
    impl_gas_costs_cal_bundles!(bundle6);
    impl_gas_costs_cal_bundles!(bundle7);
    impl_gas_costs_cal_bundles!(bundle8);
    impl_gas_costs_cal_bundles!(bundle9);
    impl_gas_costs_cal_bundles!(bundle10);
    impl_gas_costs_cal_bundles!(bundle11);
    impl_gas_costs_cal_bundles!(bundle12);
    impl_gas_costs_cal_bundles!(bundle13);
    impl_gas_costs_cal_bundles!(bundle14);
    impl_gas_costs_cal_bundles!(bundle15);
    impl_gas_costs_cal_bundles!(bundle16);
    impl_gas_costs_cal_bundles!(bundle17);
    impl_gas_costs_cal_bundles!(bundle18);
    impl_gas_costs_cal_bundles!(bundle19);
    impl_gas_costs_cal_bundles!(bundle20);
    impl_gas_costs_cal_bundles!(bundle21);
    impl_gas_costs_cal_bundles!(bundle22);
    impl_gas_costs_cal_bundles!(bundle23);
    impl_gas_costs_cal_bundles!(bundle24);
    impl_gas_costs_cal_bundles!(bundle25);
}

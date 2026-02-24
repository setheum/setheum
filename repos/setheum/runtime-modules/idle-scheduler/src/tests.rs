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

#![cfg(test)]

use super::*;
use crate::mock::{IdleScheduler, *};
use frame_support::assert_ok;

// Can schedule tasks
#[test]
fn can_schedule_tasks() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(Tasks::<Runtime>::get(0), None);

		assert_ok!(IdleScheduler::schedule_task(
			Origin::root(),
			ScheduledTasks::BalancesTask(BalancesTask::OnIdle)
		));
		assert_eq!(
			Tasks::<Runtime>::get(0),
			Some(ScheduledTasks::BalancesTask(BalancesTask::OnIdle))
		);

		assert_eq!(Tasks::<Runtime>::get(2), None);
	});
}

// can process tasks up to weight limit
#[test]
fn can_process_tasks_up_to_weight_limit() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(IdleScheduler::schedule_task(
			Origin::root(),
			ScheduledTasks::BalancesTask(BalancesTask::OnIdle)
		));
		assert_ok!(IdleScheduler::schedule_task(
			Origin::root(),
			ScheduledTasks::BalancesTask(BalancesTask::OnIdle)
		));

// Given enough weights for only 2 tasks: MinimumWeightRemainInBlock::get() + BASE_WEIGHT*2
		IdleScheduler::on_idle(0, 100_002_000_000);

// Due to hashing, excution is not guaranteed to be in order.
		assert_eq!(
			Tasks::<Runtime>::get(0),
			None
		);
		assert_eq!(Tasks::<Runtime>::get(1), None);
		assert_eq!(Tasks::<Runtime>::get(2), None);

		IdleScheduler::on_idle(0, 100_000_000_000);
		assert_eq!(
			Tasks::<Runtime>::get(0),
			None
		);

		IdleScheduler::on_idle(0, 100_001_000_000);
		assert_eq!(Tasks::<Runtime>::get(0), None);
	});
}

// can increment next task ID
#[test]
fn can_increment_next_task_id() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(NextTaskId::<Runtime>::get(), 0);
		assert_ok!(IdleScheduler::schedule_task(
			Origin::root(),
			ScheduledTasks::BalancesTask(BalancesTask::OnIdle)
		));

		assert_eq!(NextTaskId::<Runtime>::get(), 1);
	});
}

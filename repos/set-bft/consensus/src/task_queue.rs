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

use std::{
    cmp::Ordering,
    collections::{binary_heap::PeekMut, BinaryHeap},
    fmt::{Debug, Formatter},
    time,
    time::Duration,
};

#[derive(Clone, Eq, PartialEq)]
struct ScheduledTask<T: Eq> {
    task: T,
    scheduled_time: time::Instant,
}

impl<T: Eq> PartialOrd for ScheduledTask<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq> Ord for ScheduledTask<T> {
    /// Compare tasks so that earlier times come first in a max-heap.
    fn cmp(&self, other: &Self) -> Ordering {
        other.scheduled_time.cmp(&self.scheduled_time)
    }
}

#[derive(Clone, Default)]
pub struct TaskQueue<T: Eq + PartialEq> {
    queue: BinaryHeap<ScheduledTask<T>>,
}

impl<T: Eq + PartialEq> Debug for TaskQueue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskQueue")
            .field("task count", &self.queue.len())
            .finish()
    }
}

/// Implements a queue allowing for scheduling tasks for some time in the future.
///
/// Note that this queue is passive - nothing will happen until you call `pop_due_task`.
impl<T: Eq> TaskQueue<T> {
    /// Creates an empty queue.
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
        }
    }

    /// Schedules `task` for as soon as possible.
    pub fn schedule_now(&mut self, task: T) {
        self.schedule(task, time::Instant::now());
    }

    /// Schedules `task` for execution after `delay`.
    pub fn schedule_in(&mut self, task: T, delay: Duration) {
        self.schedule(task, time::Instant::now() + delay)
    }

    /// Schedules `task` for execution at `scheduled_time`.
    pub fn schedule(&mut self, task: T, scheduled_time: time::Instant) {
        self.queue.push(ScheduledTask {
            task,
            scheduled_time,
        })
    }

    /// Returns `Some(task)` if `task` is the most overdue task, and `None` if there are no overdue
    /// tasks.
    pub fn pop_due_task(&mut self) -> Option<T> {
        let scheduled_task = self.queue.peek_mut()?;

        if scheduled_task.scheduled_time <= time::Instant::now() {
            Some(PeekMut::pop(scheduled_task).task)
        } else {
            None
        }
    }

    /// Returns an iterator over all pending tasks.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.queue.iter().map(|x| &x.task)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_scheduling() {
        let mut q = TaskQueue::new();
        q.schedule_now(1);
        q.schedule_in(2, Duration::from_millis(5));
        q.schedule_in(3, Duration::from_millis(30));

        thread::sleep(Duration::from_millis(10));

        assert_eq!(Some(1), q.pop_due_task());
        assert_eq!(Some(2), q.pop_due_task());
        assert_eq!(None, q.pop_due_task());
    }
}

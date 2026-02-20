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

// Copyright (C) Use Ink (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::event::event_metadata_derive;

#[test]
fn unit_struct_works() {
    crate::test_derive! {
        event_metadata_derive {
            #[derive(ink::Event, scale::Encode)]
            struct UnitStruct;
        }
        expands to {
            const _: () = {
                impl ::ink::metadata::EventMetadata for UnitStruct {
                    const MODULE_PATH: &'static str = ::core::module_path!();

                    fn event_spec() -> ::ink::metadata::EventSpec {
                        #[::ink::metadata::linkme::distributed_slice(::ink::metadata::EVENTS)]
                        #[linkme(crate = ::ink::metadata::linkme)]
                        static EVENT_METADATA: fn() -> ::ink::metadata::EventSpec =
                            <UnitStruct as ::ink::metadata::EventMetadata>::event_spec;

                        ::ink::metadata::EventSpec::new(::core::stringify!(UnitStruct))
                            .module_path(::core::module_path!())
                            .signature_topic(<Self as ::ink::env::Event>::SIGNATURE_TOPIC)
                            .args([])
                            .docs([])
                            .done()
                    }
                }
            };
        }
    }
}

#[test]
fn struct_with_fields_no_topics() {
    crate::test_derive! {
        event_metadata_derive {
            #[derive(ink::Event, scale::Encode)]
            struct Event {
                field_1: u32,
                field_2: u64,
                field_3: u128,
            }
        }
        expands to {
            const _: () = {
                impl ::ink::metadata::EventMetadata for Event {
                    const MODULE_PATH: &'static str = ::core::module_path!();

                    fn event_spec() -> ::ink::metadata::EventSpec {
                        #[::ink::metadata::linkme::distributed_slice(::ink::metadata::EVENTS)]
                        #[linkme(crate = ::ink::metadata::linkme)]
                        static EVENT_METADATA: fn() -> ::ink::metadata::EventSpec =
                            <Event as ::ink::metadata::EventMetadata>::event_spec;

                        ::ink::metadata::EventSpec::new(::core::stringify!(Event))
                            .module_path(::core::module_path!())
                            .signature_topic(<Self as ::ink::env::Event>::SIGNATURE_TOPIC)
                            .args([
                                ::ink::metadata::EventParamSpec::new(::core::stringify!(field_1))
                                    .of_type(::ink::metadata::TypeSpec::with_name_segs::<u32, _>(
                                        ::core::iter::Iterator::map(
                                            ::core::iter::IntoIterator::into_iter([::core::stringify!(u32)]),
                                            ::core::convert::AsRef::as_ref
                                        )
                                    ))
                                    .indexed(false)
                                    .docs([])
                                    .done(),
                                ::ink::metadata::EventParamSpec::new(::core::stringify!(field_2))
                                    .of_type(::ink::metadata::TypeSpec::with_name_segs::<u64, _>(
                                        ::core::iter::Iterator::map(
                                            ::core::iter::IntoIterator::into_iter([::core::stringify!(u64)]),
                                            ::core::convert::AsRef::as_ref
                                        )
                                    ))
                                    .indexed(false)
                                    .docs([])
                                    .done(),
                                ::ink::metadata::EventParamSpec::new(::core::stringify!(field_3))
                                    .of_type(::ink::metadata::TypeSpec::with_name_segs::<u128, _>(
                                        ::core::iter::Iterator::map(
                                            ::core::iter::IntoIterator::into_iter([::core::stringify!(u128)]),
                                            ::core::convert::AsRef::as_ref
                                        )
                                    ))
                                    .indexed(false)
                                    .docs([])
                                    .done()
                            ])
                            .docs([])
                            .done()
                    }
                }
            };
        }
    }
}

#[test]
fn struct_with_fields_and_some_topics() {
    crate::test_derive! {
        event_metadata_derive {
            #[derive(ink::Event, scale::Encode)]
            struct Event {
                field_1: u32,
                #[ink(topic)]
                field_2: u64,
                #[ink(topic)]
                field_3: u128,
            }
        }
        expands to {
            const _: () = {
                impl ::ink::metadata::EventMetadata for Event {
                    const MODULE_PATH: &'static str = ::core::module_path!();

                    fn event_spec() -> ::ink::metadata::EventSpec {
                        #[::ink::metadata::linkme::distributed_slice(::ink::metadata::EVENTS)]
                        #[linkme(crate = ::ink::metadata::linkme)]
                        static EVENT_METADATA: fn() -> ::ink::metadata::EventSpec =
                            <Event as ::ink::metadata::EventMetadata>::event_spec;

                        ::ink::metadata::EventSpec::new(::core::stringify!(Event))
                            .module_path(::core::module_path!())
                            .signature_topic(<Self as ::ink::env::Event>::SIGNATURE_TOPIC)
                            .args([
                                ::ink::metadata::EventParamSpec::new(::core::stringify!(field_1))
                                    .of_type(::ink::metadata::TypeSpec::with_name_segs::<u32, _>(
                                        ::core::iter::Iterator::map(
                                            ::core::iter::IntoIterator::into_iter([::core::stringify!(u32)]),
                                            ::core::convert::AsRef::as_ref
                                        )
                                    ))
                                    .indexed(false)
                                    .docs([])
                                    .done(),
                                ::ink::metadata::EventParamSpec::new(::core::stringify!(field_2))
                                    .of_type(::ink::metadata::TypeSpec::with_name_segs::<u64, _>(
                                        ::core::iter::Iterator::map(
                                            ::core::iter::IntoIterator::into_iter([::core::stringify!(u64)]),
                                            ::core::convert::AsRef::as_ref
                                        )
                                    ))
                                    .indexed(true)
                                    .docs([])
                                    .done(),
                                ::ink::metadata::EventParamSpec::new(::core::stringify!(field_3))
                                    .of_type(::ink::metadata::TypeSpec::with_name_segs::<u128, _>(
                                        ::core::iter::Iterator::map(
                                            ::core::iter::IntoIterator::into_iter([::core::stringify!(u128)]),
                                            ::core::convert::AsRef::as_ref
                                        )
                                    ))
                                    .indexed(true)
                                    .docs([])
                                    .done()
                            ])
                            .docs([])
                            .done()
                    }
                }
            };
        }
    }
}

#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2024::*;
#[macro_use]
extern crate core;
use sails_rs::{prelude::*, cell::RefCell};
pub mod services {
    pub mod contract_service {
        use sails_rs::{prelude::*, cell::RefCell};
        #[codec(crate = sails_rs::scale_codec)]
        #[scale_info(crate = sails_rs::scale_info)]
        pub enum ContractEvent {
            Hello(ActorId),
            ValueReceived(u128),
            ValueSent(u128),
            Incremented,
            Decremented,
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl sails_rs::scale_codec::Encode for ContractEvent {
                fn size_hint(&self) -> usize {
                    1_usize
                        + match *self {
                            ContractEvent::Hello(ref aa) => {
                                0_usize
                                    .saturating_add(
                                        sails_rs::scale_codec::Encode::size_hint(aa),
                                    )
                            }
                            ContractEvent::ValueReceived(ref aa) => {
                                0_usize
                                    .saturating_add(
                                        sails_rs::scale_codec::Encode::size_hint(aa),
                                    )
                            }
                            ContractEvent::ValueSent(ref aa) => {
                                0_usize
                                    .saturating_add(
                                        sails_rs::scale_codec::Encode::size_hint(aa),
                                    )
                            }
                            ContractEvent::Incremented => 0_usize,
                            ContractEvent::Decremented => 0_usize,
                            _ => 0_usize,
                        }
                }
                fn encode_to<
                    __CodecOutputEdqy: sails_rs::scale_codec::Output
                        + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    #[automatically_derived]
                    const _: () = {
                        #[allow(clippy::unnecessary_cast)]
                        #[allow(clippy::cast_possible_truncation)]
                        const indices: [(usize, &'static str); 5usize] = [
                            ((0usize) as ::core::primitive::usize, "Hello"),
                            ((1usize) as ::core::primitive::usize, "ValueReceived"),
                            ((2usize) as ::core::primitive::usize, "ValueSent"),
                            ((3usize) as ::core::primitive::usize, "Incremented"),
                            ((4usize) as ::core::primitive::usize, "Decremented"),
                        ];
                        const fn search_for_invalid_index(
                            array: &[(usize, &'static str); 5usize],
                        ) -> (bool, usize) {
                            let mut i = 0;
                            while i < 5usize {
                                if array[i].0 > 255 {
                                    return (true, i);
                                }
                                i += 1;
                            }
                            (false, 0)
                        }
                        const INVALID_INDEX: (bool, usize) = search_for_invalid_index(
                            &indices,
                        );
                        if INVALID_INDEX.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper("Found variant `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].1,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` with invalid index: `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        indices[INVALID_INDEX.1].0,
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Max supported index is 255.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                #[cold]
                                #[track_caller]
                                #[inline(never)]
                                #[rustc_const_panic_str]
                                #[rustc_do_not_const_check]
                                const fn panic_cold_display<T: ::core::fmt::Display>(
                                    arg: &T,
                                ) -> ! {
                                    ::core::panicking::panic_display(arg)
                                }
                                panic_cold_display(&msg);
                            };
                        }
                        const fn duplicate_info(
                            array: &[(usize, &'static str); 5usize],
                        ) -> (bool, usize, usize) {
                            let len = 5usize;
                            let mut i = 0usize;
                            while i < len {
                                let mut j = i + 1;
                                while j < len {
                                    if array[i].0 == array[j].0 {
                                        return (true, i, j);
                                    }
                                    j += 1;
                                }
                                i += 1;
                            }
                            (false, 0, 0)
                        }
                        const DUP_INFO: (bool, usize, usize) = duplicate_info(&indices);
                        if DUP_INFO.0 {
                            let msg = ::const_format::pmr::__AssertStr {
                                x: {
                                    use ::const_format::__cf_osRcTFl4A;
                                    ({
                                        #[doc(hidden)]
                                        #[allow(unused_mut, non_snake_case)]
                                        const CONCATP_NHPMWYD3NJA: &[__cf_osRcTFl4A::pmr::PArgument] = {
                                            let fmt = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;
                                            &[
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "Found variants that have duplicate indexes. Both `",
                                                    )
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` and `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.2].1)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper("` have the index `")
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(indices[DUP_INFO.1].0)
                                                    .to_pargument_display(fmt),
                                                __cf_osRcTFl4A::pmr::PConvWrapper(
                                                        "`. Use different indexes for each variant.",
                                                    )
                                                    .to_pargument_display(fmt),
                                            ]
                                        };
                                        {
                                            #[doc(hidden)]
                                            const ARR_LEN: usize = ::const_format::pmr::PArgument::calc_len(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            const CONCAT_ARR: &::const_format::pmr::LenAndArray<
                                                [u8; ARR_LEN],
                                            > = &::const_format::pmr::__priv_concatenate(
                                                CONCATP_NHPMWYD3NJA,
                                            );
                                            #[doc(hidden)]
                                            #[allow(clippy::transmute_ptr_to_ptr)]
                                            const CONCAT_STR: &str = unsafe {
                                                let slice = ::const_format::pmr::transmute::<
                                                    &[u8; ARR_LEN],
                                                    &[u8; CONCAT_ARR.len],
                                                >(&CONCAT_ARR.array);
                                                {
                                                    let bytes: &'static [::const_format::pmr::u8] = slice;
                                                    let string: &'static ::const_format::pmr::str = {
                                                        ::const_format::__hidden_utils::PtrToRef {
                                                            ptr: bytes as *const [::const_format::pmr::u8] as *const str,
                                                        }
                                                            .reff
                                                    };
                                                    string
                                                }
                                            };
                                            CONCAT_STR
                                        }
                                    })
                                },
                            }
                                .x;
                            {
                                #[cold]
                                #[track_caller]
                                #[inline(never)]
                                #[rustc_const_panic_str]
                                #[rustc_do_not_const_check]
                                const fn panic_cold_display<T: ::core::fmt::Display>(
                                    arg: &T,
                                ) -> ! {
                                    ::core::panicking::panic_display(arg)
                                }
                                panic_cold_display(&msg);
                            };
                        }
                    };
                    match *self {
                        ContractEvent::Hello(ref aa) => {
                            #[allow(clippy::unnecessary_cast)]
                            __codec_dest_edqy
                                .push_byte((0usize) as ::core::primitive::u8);
                            sails_rs::scale_codec::Encode::encode_to(
                                aa,
                                __codec_dest_edqy,
                            );
                        }
                        ContractEvent::ValueReceived(ref aa) => {
                            #[allow(clippy::unnecessary_cast)]
                            __codec_dest_edqy
                                .push_byte((1usize) as ::core::primitive::u8);
                            sails_rs::scale_codec::Encode::encode_to(
                                aa,
                                __codec_dest_edqy,
                            );
                        }
                        ContractEvent::ValueSent(ref aa) => {
                            #[allow(clippy::unnecessary_cast)]
                            __codec_dest_edqy
                                .push_byte((2usize) as ::core::primitive::u8);
                            sails_rs::scale_codec::Encode::encode_to(
                                aa,
                                __codec_dest_edqy,
                            );
                        }
                        ContractEvent::Incremented => {
                            #[allow(clippy::unnecessary_cast)]
                            #[allow(clippy::cast_possible_truncation)]
                            __codec_dest_edqy
                                .push_byte((3usize) as ::core::primitive::u8);
                        }
                        ContractEvent::Decremented => {
                            #[allow(clippy::unnecessary_cast)]
                            #[allow(clippy::cast_possible_truncation)]
                            __codec_dest_edqy
                                .push_byte((4usize) as ::core::primitive::u8);
                        }
                        _ => {}
                    }
                }
            }
            #[automatically_derived]
            impl sails_rs::scale_codec::EncodeLike for ContractEvent {}
        };
        #[allow(
            non_upper_case_globals,
            deprecated,
            unused_attributes,
            unused_qualifications
        )]
        const _: () = {
            impl sails_rs::scale_info::TypeInfo for ContractEvent {
                type Identity = Self;
                fn type_info() -> sails_rs::scale_info::Type {
                    sails_rs::scale_info::Type::builder()
                        .path(
                            sails_rs::scale_info::Path::new_with_replace(
                                "ContractEvent",
                                "contract_app::services::contract_service",
                                &[],
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .variant(
                            sails_rs::scale_info::build::Variants::new()
                                .variant(
                                    "Hello",
                                    |v| {
                                        v
                                            .index(0usize as ::core::primitive::u8)
                                            .fields(
                                                sails_rs::scale_info::build::Fields::unnamed()
                                                    .field(|f| f.ty::<ActorId>().type_name("ActorId")),
                                            )
                                    },
                                )
                                .variant(
                                    "ValueReceived",
                                    |v| {
                                        v
                                            .index(1usize as ::core::primitive::u8)
                                            .fields(
                                                sails_rs::scale_info::build::Fields::unnamed()
                                                    .field(|f| f.ty::<u128>().type_name("u128")),
                                            )
                                    },
                                )
                                .variant(
                                    "ValueSent",
                                    |v| {
                                        v
                                            .index(2usize as ::core::primitive::u8)
                                            .fields(
                                                sails_rs::scale_info::build::Fields::unnamed()
                                                    .field(|f| f.ty::<u128>().type_name("u128")),
                                            )
                                    },
                                )
                                .variant(
                                    "Incremented",
                                    |v| v.index(3usize as ::core::primitive::u8),
                                )
                                .variant(
                                    "Decremented",
                                    |v| v.index(4usize as ::core::primitive::u8),
                                ),
                        )
                }
            }
        };
        impl sails_rs::SailsEvent for ContractEvent {
            fn encoded_event_name(&self) -> &'static [u8] {
                match self {
                    ContractEvent::Hello(..) => &[20u8, 72u8, 101u8, 108u8, 108u8, 111u8],
                    ContractEvent::ValueReceived(..) => {
                        &[
                            52u8, 86u8, 97u8, 108u8, 117u8, 101u8, 82u8, 101u8, 99u8,
                            101u8, 105u8, 118u8, 101u8, 100u8,
                        ]
                    }
                    ContractEvent::ValueSent(..) => {
                        &[
                            36u8, 86u8, 97u8, 108u8, 117u8, 101u8, 83u8, 101u8, 110u8,
                            116u8,
                        ]
                    }
                    ContractEvent::Incremented => {
                        &[
                            44u8, 73u8, 110u8, 99u8, 114u8, 101u8, 109u8, 101u8, 110u8,
                            116u8, 101u8, 100u8,
                        ]
                    }
                    ContractEvent::Decremented => {
                        &[
                            44u8, 68u8, 101u8, 99u8, 114u8, 101u8, 109u8, 101u8, 110u8,
                            116u8, 101u8, 100u8,
                        ]
                    }
                }
            }
            fn skip_bytes() -> usize {
                1
            }
        }
        pub struct CounterData {
            counter: u64,
        }
        #[automatically_derived]
        impl ::core::default::Default for CounterData {
            #[inline]
            fn default() -> CounterData {
                CounterData {
                    counter: ::core::default::Default::default(),
                }
            }
        }
        pub struct ContractService<'a> {
            state: &'a RefCell<CounterData>,
        }
        impl<'a> ContractService<'a> {
            pub fn new(state: &'a RefCell<CounterData>) -> Self {
                Self { state }
            }
        }
        pub struct ContractServiceExposure<T> {
            route: &'static [u8],
            inner: T,
        }
        impl<T: sails_rs::meta::ServiceMeta> sails_rs::gstd::services::Exposure
        for ContractServiceExposure<T> {
            fn route(&self) -> &'static [u8] {
                self.route
            }
            fn check_asyncness(input: &[u8]) -> Option<bool> {
                use sails_rs::gstd::InvocationIo;
                use sails_rs::gstd::services::{Service, Exposure};
                if !T::ASYNC {
                    return Some(false);
                }
                if let Ok(is_async) = contract_service_meta::__CounterValueParams::check_asyncness(
                    input,
                ) {
                    return Some(is_async);
                }
                if let Ok(is_async) = contract_service_meta::__DecrementParams::check_asyncness(
                    input,
                ) {
                    return Some(is_async);
                }
                if let Ok(is_async) = contract_service_meta::__GetValueParams::check_asyncness(
                    input,
                ) {
                    return Some(is_async);
                }
                if let Ok(is_async) = contract_service_meta::__HelloServiceParams::check_asyncness(
                    input,
                ) {
                    return Some(is_async);
                }
                if let Ok(is_async) = contract_service_meta::__IncrementParams::check_asyncness(
                    input,
                ) {
                    return Some(is_async);
                }
                if let Ok(is_async) = contract_service_meta::__SendValueParams::check_asyncness(
                    input,
                ) {
                    return Some(is_async);
                }
                None
            }
        }
        impl<T: sails_rs::meta::ServiceMeta> sails_rs::gstd::services::ExposureWithEvents
        for ContractServiceExposure<T> {
            type Events = ContractEvent;
        }
        impl<T> core::ops::Deref for ContractServiceExposure<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
        impl<T> core::ops::DerefMut for ContractServiceExposure<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
        impl ContractServiceExposure<ContractService<'_>> {
            pub fn hello_service(&mut self) -> String {
                let msg_source = Syscall::message_source();
                self.emit_event(ContractEvent::Hello(Syscall::message_source()))
                    .unwrap();
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("Hello {0:?}", msg_source))
                })
            }
            pub fn send_value(&mut self) -> String {
                let value = Syscall::message_value();
                self.emit_event(ContractEvent::ValueReceived(value)).unwrap();
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("Value get: {0}", value))
                })
            }
            pub fn get_value(&mut self, to_return: u128) -> CommandReply<String> {
                let contract_tokens = Syscall::value_available();
                if contract_tokens > to_return {
                    self.emit_event(ContractEvent::ValueSent(to_return)).unwrap();
                    CommandReply::new(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("Value returned: {0}", to_return),
                                )
                            }),
                        )
                        .with_value(to_return)
                } else {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("Cant transfer tokens"),
                        );
                    };
                }
            }
            pub fn increment(&mut self) -> u64 {
                let mut state = self.state.borrow_mut();
                self.emit_event(ContractEvent::Incremented).unwrap();
                state.counter += 1;
                state.counter
            }
            pub fn decrement(&mut self) -> Result<u64, String> {
                let mut state = self.state.borrow_mut();
                state.counter = state
                    .counter
                    .checked_sub(1)
                    .ok_or("Counter can not be negative!".to_string())?;
                self.emit_event(ContractEvent::Decremented).unwrap();
                Ok(state.counter)
            }
            pub fn counter_value(&self) -> u64 {
                self.state.borrow().counter
            }
            pub fn check_asyncness(&self, input: &[u8]) -> Option<bool> {
                <Self as sails_rs::gstd::services::Exposure>::check_asyncness(input)
            }
            pub fn try_handle(
                mut self,
                input: &[u8],
                result_handler: fn(&[u8], u128),
            ) -> Option<()> {
                use sails_rs::gstd::InvocationIo;
                use sails_rs::gstd::services::{Service, Exposure};
                if let Ok(request) = contract_service_meta::__CounterValueParams::decode_params(
                    input,
                ) {
                    let result = self.counter_value();
                    let value = 0u128;
                    if !contract_service_meta::__CounterValueParams::is_empty_tuple::<
                        u64,
                    >() {
                        contract_service_meta::__CounterValueParams::with_optimized_encode(
                            &result,
                            self.route().as_ref(),
                            |encoded_result| result_handler(encoded_result, value),
                        );
                    }
                    return Some(());
                }
                if let Ok(request) = contract_service_meta::__DecrementParams::decode_params(
                    input,
                ) {
                    let result = self.decrement().unwrap();
                    let value = 0u128;
                    if !contract_service_meta::__DecrementParams::is_empty_tuple::<
                        u64,
                    >() {
                        contract_service_meta::__DecrementParams::with_optimized_encode(
                            &result,
                            self.route().as_ref(),
                            |encoded_result| result_handler(encoded_result, value),
                        );
                    }
                    return Some(());
                }
                if let Ok(request) = contract_service_meta::__GetValueParams::decode_params(
                    input,
                ) {
                    let command_reply: CommandReply<String> = self
                        .get_value(request.to_return)
                        .into();
                    let (result, value) = command_reply.to_tuple();
                    if !contract_service_meta::__GetValueParams::is_empty_tuple::<
                        String,
                    >() {
                        contract_service_meta::__GetValueParams::with_optimized_encode(
                            &result,
                            self.route().as_ref(),
                            |encoded_result| result_handler(encoded_result, value),
                        );
                    }
                    return Some(());
                }
                if let Ok(request) = contract_service_meta::__HelloServiceParams::decode_params(
                    input,
                ) {
                    let result = self.hello_service();
                    let value = 0u128;
                    if !contract_service_meta::__HelloServiceParams::is_empty_tuple::<
                        String,
                    >() {
                        contract_service_meta::__HelloServiceParams::with_optimized_encode(
                            &result,
                            self.route().as_ref(),
                            |encoded_result| result_handler(encoded_result, value),
                        );
                    }
                    return Some(());
                }
                if let Ok(request) = contract_service_meta::__IncrementParams::decode_params(
                    input,
                ) {
                    let result = self.increment();
                    let value = 0u128;
                    if !contract_service_meta::__IncrementParams::is_empty_tuple::<
                        u64,
                    >() {
                        contract_service_meta::__IncrementParams::with_optimized_encode(
                            &result,
                            self.route().as_ref(),
                            |encoded_result| result_handler(encoded_result, value),
                        );
                    }
                    return Some(());
                }
                if let Ok(request) = contract_service_meta::__SendValueParams::decode_params(
                    input,
                ) {
                    let result = self.send_value();
                    let value = 0u128;
                    if !contract_service_meta::__SendValueParams::is_empty_tuple::<
                        String,
                    >() {
                        contract_service_meta::__SendValueParams::with_optimized_encode(
                            &result,
                            self.route().as_ref(),
                            |encoded_result| result_handler(encoded_result, value),
                        );
                    }
                    return Some(());
                }
                None
            }
            pub async fn try_handle_async(
                mut self,
                input: &[u8],
                result_handler: fn(&[u8], u128),
            ) -> Option<()> {
                use sails_rs::gstd::InvocationIo;
                use sails_rs::gstd::services::{Service, Exposure};
                None
            }
            pub fn emit_event(
                &self,
                event: ContractEvent,
            ) -> sails_rs::errors::Result<()> {
                use sails_rs::gstd::services::ExposureWithEvents;
                self.emitter().emit_event(event)
            }
        }
        impl sails_rs::gstd::services::Service for ContractService<'_> {
            type Exposure = ContractServiceExposure<Self>;
            fn expose(self, route: &'static [u8]) -> Self::Exposure {
                Self::Exposure {
                    route,
                    inner: self,
                }
            }
        }
        impl sails_rs::meta::ServiceMeta for ContractService<'_> {
            type CommandsMeta = contract_service_meta::CommandsMeta;
            type QueriesMeta = contract_service_meta::QueriesMeta;
            type EventsMeta = contract_service_meta::EventsMeta;
            const BASE_SERVICES: &'static [sails_rs::meta::AnyServiceMetaFn] = &[];
            const ASYNC: bool = false;
        }
        mod contract_service_meta {
            use super::*;
            use sails_rs::{Decode, TypeInfo};
            use sails_rs::gstd::InvocationIo;
            #[codec(crate = sails_rs::scale_codec)]
            #[scale_info(crate = sails_rs::scale_info)]
            pub struct __CounterValueParams {}
            #[allow(deprecated)]
            const _: () = {
                #[automatically_derived]
                impl sails_rs::scale_codec::Decode for __CounterValueParams {
                    fn decode<__CodecInputEdqy: sails_rs::scale_codec::Input>(
                        __codec_input_edqy: &mut __CodecInputEdqy,
                    ) -> ::core::result::Result<Self, sails_rs::scale_codec::Error> {
                        ::core::result::Result::Ok(__CounterValueParams {})
                    }
                }
            };
            #[allow(
                non_upper_case_globals,
                deprecated,
                unused_attributes,
                unused_qualifications
            )]
            const _: () = {
                impl sails_rs::scale_info::TypeInfo for __CounterValueParams {
                    type Identity = Self;
                    fn type_info() -> sails_rs::scale_info::Type {
                        sails_rs::scale_info::Type::builder()
                            .path(
                                sails_rs::scale_info::Path::new_with_replace(
                                    "__CounterValueParams",
                                    "contract_app::services::contract_service::contract_service_meta",
                                    &[],
                                ),
                            )
                            .type_params(::alloc::vec::Vec::new())
                            .composite(sails_rs::scale_info::build::Fields::named())
                    }
                }
            };
            impl InvocationIo for __CounterValueParams {
                const ROUTE: &'static [u8] = &[
                    48u8, 67u8, 111u8, 117u8, 110u8, 116u8, 101u8, 114u8, 86u8, 97u8,
                    108u8, 117u8, 101u8,
                ];
                type Params = Self;
                const ASYNC: bool = false;
            }
            #[codec(crate = sails_rs::scale_codec)]
            #[scale_info(crate = sails_rs::scale_info)]
            pub struct __DecrementParams {}
            #[allow(deprecated)]
            const _: () = {
                #[automatically_derived]
                impl sails_rs::scale_codec::Decode for __DecrementParams {
                    fn decode<__CodecInputEdqy: sails_rs::scale_codec::Input>(
                        __codec_input_edqy: &mut __CodecInputEdqy,
                    ) -> ::core::result::Result<Self, sails_rs::scale_codec::Error> {
                        ::core::result::Result::Ok(__DecrementParams {})
                    }
                }
            };
            #[allow(
                non_upper_case_globals,
                deprecated,
                unused_attributes,
                unused_qualifications
            )]
            const _: () = {
                impl sails_rs::scale_info::TypeInfo for __DecrementParams {
                    type Identity = Self;
                    fn type_info() -> sails_rs::scale_info::Type {
                        sails_rs::scale_info::Type::builder()
                            .path(
                                sails_rs::scale_info::Path::new_with_replace(
                                    "__DecrementParams",
                                    "contract_app::services::contract_service::contract_service_meta",
                                    &[],
                                ),
                            )
                            .type_params(::alloc::vec::Vec::new())
                            .composite(sails_rs::scale_info::build::Fields::named())
                    }
                }
            };
            impl InvocationIo for __DecrementParams {
                const ROUTE: &'static [u8] = &[
                    36u8, 68u8, 101u8, 99u8, 114u8, 101u8, 109u8, 101u8, 110u8, 116u8,
                ];
                type Params = Self;
                const ASYNC: bool = false;
            }
            #[codec(crate = sails_rs::scale_codec)]
            #[scale_info(crate = sails_rs::scale_info)]
            pub struct __GetValueParams {
                pub(super) to_return: u128,
            }
            #[allow(deprecated)]
            const _: () = {
                #[automatically_derived]
                impl sails_rs::scale_codec::Decode for __GetValueParams {
                    fn decode<__CodecInputEdqy: sails_rs::scale_codec::Input>(
                        __codec_input_edqy: &mut __CodecInputEdqy,
                    ) -> ::core::result::Result<Self, sails_rs::scale_codec::Error> {
                        ::core::result::Result::Ok(__GetValueParams {
                            to_return: {
                                let __codec_res_edqy = <u128 as sails_rs::scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `__GetValueParams::to_return`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                }
            };
            #[allow(
                non_upper_case_globals,
                deprecated,
                unused_attributes,
                unused_qualifications
            )]
            const _: () = {
                impl sails_rs::scale_info::TypeInfo for __GetValueParams {
                    type Identity = Self;
                    fn type_info() -> sails_rs::scale_info::Type {
                        sails_rs::scale_info::Type::builder()
                            .path(
                                sails_rs::scale_info::Path::new_with_replace(
                                    "__GetValueParams",
                                    "contract_app::services::contract_service::contract_service_meta",
                                    &[],
                                ),
                            )
                            .type_params(::alloc::vec::Vec::new())
                            .composite(
                                sails_rs::scale_info::build::Fields::named()
                                    .field(|f| {
                                        f.ty::<u128>().name("to_return").type_name("u128")
                                    }),
                            )
                    }
                }
            };
            impl InvocationIo for __GetValueParams {
                const ROUTE: &'static [u8] = &[
                    32u8, 71u8, 101u8, 116u8, 86u8, 97u8, 108u8, 117u8, 101u8,
                ];
                type Params = Self;
                const ASYNC: bool = false;
            }
            #[codec(crate = sails_rs::scale_codec)]
            #[scale_info(crate = sails_rs::scale_info)]
            pub struct __HelloServiceParams {}
            #[allow(deprecated)]
            const _: () = {
                #[automatically_derived]
                impl sails_rs::scale_codec::Decode for __HelloServiceParams {
                    fn decode<__CodecInputEdqy: sails_rs::scale_codec::Input>(
                        __codec_input_edqy: &mut __CodecInputEdqy,
                    ) -> ::core::result::Result<Self, sails_rs::scale_codec::Error> {
                        ::core::result::Result::Ok(__HelloServiceParams {})
                    }
                }
            };
            #[allow(
                non_upper_case_globals,
                deprecated,
                unused_attributes,
                unused_qualifications
            )]
            const _: () = {
                impl sails_rs::scale_info::TypeInfo for __HelloServiceParams {
                    type Identity = Self;
                    fn type_info() -> sails_rs::scale_info::Type {
                        sails_rs::scale_info::Type::builder()
                            .path(
                                sails_rs::scale_info::Path::new_with_replace(
                                    "__HelloServiceParams",
                                    "contract_app::services::contract_service::contract_service_meta",
                                    &[],
                                ),
                            )
                            .type_params(::alloc::vec::Vec::new())
                            .composite(sails_rs::scale_info::build::Fields::named())
                    }
                }
            };
            impl InvocationIo for __HelloServiceParams {
                const ROUTE: &'static [u8] = &[
                    48u8, 72u8, 101u8, 108u8, 108u8, 111u8, 83u8, 101u8, 114u8, 118u8,
                    105u8, 99u8, 101u8,
                ];
                type Params = Self;
                const ASYNC: bool = false;
            }
            #[codec(crate = sails_rs::scale_codec)]
            #[scale_info(crate = sails_rs::scale_info)]
            pub struct __IncrementParams {}
            #[allow(deprecated)]
            const _: () = {
                #[automatically_derived]
                impl sails_rs::scale_codec::Decode for __IncrementParams {
                    fn decode<__CodecInputEdqy: sails_rs::scale_codec::Input>(
                        __codec_input_edqy: &mut __CodecInputEdqy,
                    ) -> ::core::result::Result<Self, sails_rs::scale_codec::Error> {
                        ::core::result::Result::Ok(__IncrementParams {})
                    }
                }
            };
            #[allow(
                non_upper_case_globals,
                deprecated,
                unused_attributes,
                unused_qualifications
            )]
            const _: () = {
                impl sails_rs::scale_info::TypeInfo for __IncrementParams {
                    type Identity = Self;
                    fn type_info() -> sails_rs::scale_info::Type {
                        sails_rs::scale_info::Type::builder()
                            .path(
                                sails_rs::scale_info::Path::new_with_replace(
                                    "__IncrementParams",
                                    "contract_app::services::contract_service::contract_service_meta",
                                    &[],
                                ),
                            )
                            .type_params(::alloc::vec::Vec::new())
                            .composite(sails_rs::scale_info::build::Fields::named())
                    }
                }
            };
            impl InvocationIo for __IncrementParams {
                const ROUTE: &'static [u8] = &[
                    36u8, 73u8, 110u8, 99u8, 114u8, 101u8, 109u8, 101u8, 110u8, 116u8,
                ];
                type Params = Self;
                const ASYNC: bool = false;
            }
            #[codec(crate = sails_rs::scale_codec)]
            #[scale_info(crate = sails_rs::scale_info)]
            pub struct __SendValueParams {}
            #[allow(deprecated)]
            const _: () = {
                #[automatically_derived]
                impl sails_rs::scale_codec::Decode for __SendValueParams {
                    fn decode<__CodecInputEdqy: sails_rs::scale_codec::Input>(
                        __codec_input_edqy: &mut __CodecInputEdqy,
                    ) -> ::core::result::Result<Self, sails_rs::scale_codec::Error> {
                        ::core::result::Result::Ok(__SendValueParams {})
                    }
                }
            };
            #[allow(
                non_upper_case_globals,
                deprecated,
                unused_attributes,
                unused_qualifications
            )]
            const _: () = {
                impl sails_rs::scale_info::TypeInfo for __SendValueParams {
                    type Identity = Self;
                    fn type_info() -> sails_rs::scale_info::Type {
                        sails_rs::scale_info::Type::builder()
                            .path(
                                sails_rs::scale_info::Path::new_with_replace(
                                    "__SendValueParams",
                                    "contract_app::services::contract_service::contract_service_meta",
                                    &[],
                                ),
                            )
                            .type_params(::alloc::vec::Vec::new())
                            .composite(sails_rs::scale_info::build::Fields::named())
                    }
                }
            };
            impl InvocationIo for __SendValueParams {
                const ROUTE: &'static [u8] = &[
                    36u8, 83u8, 101u8, 110u8, 100u8, 86u8, 97u8, 108u8, 117u8, 101u8,
                ];
                type Params = Self;
                const ASYNC: bool = false;
            }
            #[scale_info(crate = sails_rs::scale_info)]
            pub enum CommandsMeta {
                Decrement(__DecrementParams, u64),
                GetValue(__GetValueParams, String),
                HelloService(__HelloServiceParams, String),
                Increment(__IncrementParams, u64),
                SendValue(__SendValueParams, String),
            }
            #[allow(
                non_upper_case_globals,
                deprecated,
                unused_attributes,
                unused_qualifications
            )]
            const _: () = {
                impl sails_rs::scale_info::TypeInfo for CommandsMeta {
                    type Identity = Self;
                    fn type_info() -> sails_rs::scale_info::Type {
                        sails_rs::scale_info::Type::builder()
                            .path(
                                sails_rs::scale_info::Path::new_with_replace(
                                    "CommandsMeta",
                                    "contract_app::services::contract_service::contract_service_meta",
                                    &[],
                                ),
                            )
                            .type_params(::alloc::vec::Vec::new())
                            .variant(
                                sails_rs::scale_info::build::Variants::new()
                                    .variant(
                                        "Decrement",
                                        |v| {
                                            v
                                                .index(0usize as ::core::primitive::u8)
                                                .fields(
                                                    sails_rs::scale_info::build::Fields::unnamed()
                                                        .field(|f| {
                                                            f.ty::<__DecrementParams>().type_name("__DecrementParams")
                                                        })
                                                        .field(|f| f.ty::<u64>().type_name("u64")),
                                                )
                                        },
                                    )
                                    .variant(
                                        "GetValue",
                                        |v| {
                                            v
                                                .index(1usize as ::core::primitive::u8)
                                                .fields(
                                                    sails_rs::scale_info::build::Fields::unnamed()
                                                        .field(|f| {
                                                            f.ty::<__GetValueParams>().type_name("__GetValueParams")
                                                        })
                                                        .field(|f| f.ty::<String>().type_name("String")),
                                                )
                                        },
                                    )
                                    .variant(
                                        "HelloService",
                                        |v| {
                                            v
                                                .index(2usize as ::core::primitive::u8)
                                                .fields(
                                                    sails_rs::scale_info::build::Fields::unnamed()
                                                        .field(|f| {
                                                            f
                                                                .ty::<__HelloServiceParams>()
                                                                .type_name("__HelloServiceParams")
                                                        })
                                                        .field(|f| f.ty::<String>().type_name("String")),
                                                )
                                        },
                                    )
                                    .variant(
                                        "Increment",
                                        |v| {
                                            v
                                                .index(3usize as ::core::primitive::u8)
                                                .fields(
                                                    sails_rs::scale_info::build::Fields::unnamed()
                                                        .field(|f| {
                                                            f.ty::<__IncrementParams>().type_name("__IncrementParams")
                                                        })
                                                        .field(|f| f.ty::<u64>().type_name("u64")),
                                                )
                                        },
                                    )
                                    .variant(
                                        "SendValue",
                                        |v| {
                                            v
                                                .index(4usize as ::core::primitive::u8)
                                                .fields(
                                                    sails_rs::scale_info::build::Fields::unnamed()
                                                        .field(|f| {
                                                            f.ty::<__SendValueParams>().type_name("__SendValueParams")
                                                        })
                                                        .field(|f| f.ty::<String>().type_name("String")),
                                                )
                                        },
                                    ),
                            )
                    }
                }
            };
            #[scale_info(crate = sails_rs::scale_info)]
            pub enum QueriesMeta {
                CounterValue(__CounterValueParams, u64),
            }
            #[allow(
                non_upper_case_globals,
                deprecated,
                unused_attributes,
                unused_qualifications
            )]
            const _: () = {
                impl sails_rs::scale_info::TypeInfo for QueriesMeta {
                    type Identity = Self;
                    fn type_info() -> sails_rs::scale_info::Type {
                        sails_rs::scale_info::Type::builder()
                            .path(
                                sails_rs::scale_info::Path::new_with_replace(
                                    "QueriesMeta",
                                    "contract_app::services::contract_service::contract_service_meta",
                                    &[],
                                ),
                            )
                            .type_params(::alloc::vec::Vec::new())
                            .variant(
                                sails_rs::scale_info::build::Variants::new()
                                    .variant(
                                        "CounterValue",
                                        |v| {
                                            v
                                                .index(0usize as ::core::primitive::u8)
                                                .fields(
                                                    sails_rs::scale_info::build::Fields::unnamed()
                                                        .field(|f| {
                                                            f
                                                                .ty::<__CounterValueParams>()
                                                                .type_name("__CounterValueParams")
                                                        })
                                                        .field(|f| f.ty::<u64>().type_name("u64")),
                                                )
                                        },
                                    ),
                            )
                    }
                }
            };
            #[scale_info(crate = sails_rs::scale_info)]
            pub enum NoEvents {}
            #[allow(
                non_upper_case_globals,
                deprecated,
                unused_attributes,
                unused_qualifications
            )]
            const _: () = {
                impl sails_rs::scale_info::TypeInfo for NoEvents {
                    type Identity = Self;
                    fn type_info() -> sails_rs::scale_info::Type {
                        sails_rs::scale_info::Type::builder()
                            .path(
                                sails_rs::scale_info::Path::new_with_replace(
                                    "NoEvents",
                                    "contract_app::services::contract_service::contract_service_meta",
                                    &[],
                                ),
                            )
                            .type_params(::alloc::vec::Vec::new())
                            .variant(sails_rs::scale_info::build::Variants::new())
                    }
                }
            };
            pub type EventsMeta = ContractEvent;
        }
    }
}
use services::contract_service::{ContractService, CounterData};
pub struct ContractProgram {
    state: RefCell<CounterData>,
}
impl ContractProgram {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(Default::default()),
        }
    }
    fn __contract_svc(&self) -> ContractService<'_> {
        ContractService::new(&self.state)
    }
    pub fn contract_svc(
        &self,
    ) -> <ContractService<'_> as sails_rs::gstd::services::Service>::Exposure {
        let service = self.__contract_svc();
        let exposure = <ContractService<
            '_,
        > as sails_rs::gstd::services::Service>::expose(
            service,
            __ROUTE_CONTRACTSERVICE.as_ref(),
        );
        exposure
    }
}
const __ROUTE_CONTRACTSERVICE: [u8; 16usize] = [
    60u8, 67u8, 111u8, 110u8, 116u8, 114u8, 97u8, 99u8, 116u8, 83u8, 101u8, 114u8, 118u8,
    105u8, 99u8, 101u8,
];
impl sails_rs::meta::ProgramMeta for ContractProgram {
    type ConstructorsMeta = meta_in_program::ConstructorsMeta;
    const SERVICES: &'static [(&'static str, sails_rs::meta::AnyServiceMetaFn)] = &[
        ("ContractService", sails_rs::meta::AnyServiceMeta::new::<ContractService<'_>>),
    ];
    const ASYNC: bool = <ContractService<'_> as sails_rs::meta::ServiceMeta>::ASYNC;
}
mod meta_in_program {
    use super::*;
    use sails_rs::gstd::InvocationIo;
    #[codec(crate = sails_rs::scale_codec)]
    #[scale_info(crate = sails_rs::scale_info)]
    pub struct __NewParams {}
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl sails_rs::scale_codec::Decode for __NewParams {
            fn decode<__CodecInputEdqy: sails_rs::scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, sails_rs::scale_codec::Error> {
                ::core::result::Result::Ok(__NewParams {})
            }
        }
    };
    #[allow(
        non_upper_case_globals,
        deprecated,
        unused_attributes,
        unused_qualifications
    )]
    const _: () = {
        impl sails_rs::scale_info::TypeInfo for __NewParams {
            type Identity = Self;
            fn type_info() -> sails_rs::scale_info::Type {
                sails_rs::scale_info::Type::builder()
                    .path(
                        sails_rs::scale_info::Path::new_with_replace(
                            "__NewParams",
                            "contract_app::meta_in_program",
                            &[],
                        ),
                    )
                    .type_params(::alloc::vec::Vec::new())
                    .composite(sails_rs::scale_info::build::Fields::named())
            }
        }
    };
    impl InvocationIo for __NewParams {
        const ROUTE: &'static [u8] = &[12u8, 78u8, 101u8, 119u8];
        type Params = Self;
        const ASYNC: bool = false;
    }
    #[scale_info(crate = sails_rs::scale_info)]
    pub enum ConstructorsMeta {
        New(__NewParams),
    }
    #[allow(
        non_upper_case_globals,
        deprecated,
        unused_attributes,
        unused_qualifications
    )]
    const _: () = {
        impl sails_rs::scale_info::TypeInfo for ConstructorsMeta {
            type Identity = Self;
            fn type_info() -> sails_rs::scale_info::Type {
                sails_rs::scale_info::Type::builder()
                    .path(
                        sails_rs::scale_info::Path::new_with_replace(
                            "ConstructorsMeta",
                            "contract_app::meta_in_program",
                            &[],
                        ),
                    )
                    .type_params(::alloc::vec::Vec::new())
                    .variant(
                        sails_rs::scale_info::build::Variants::new()
                            .variant(
                                "New",
                                |v| {
                                    v
                                        .index(0usize as ::core::primitive::u8)
                                        .fields(
                                            sails_rs::scale_info::build::Fields::unnamed()
                                                .field(|f| f.ty::<__NewParams>().type_name("__NewParams")),
                                        )
                                },
                            ),
                    )
            }
        }
    };
}

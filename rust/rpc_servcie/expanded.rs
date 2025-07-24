#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use clap::Parser;
use tokio::net::TcpListener;
mod client {
    use crate::service::MathServiceClient;
    use anyhow::Result;
    use tarpc::context;
    use tarpc::serde_transport::tcp;
    use tarpc::tokio_serde::formats::Json;
    pub async fn run_client(addr: &str) -> Result<()> {
        let transport = tcp::connect(addr, Json::default).await?;
        let client = MathServiceClient::new(tarpc::client::Config::default(), transport)
            .spawn();
        let ctx = context::current();
        let sum = client.add(ctx.clone(), 5, 3).await?;
        {
            ::std::io::_print(format_args!("5 + 3 = {0}\n", sum));
        };
        for n in [5, 10, 25] {
            match client.factorial(ctx.clone(), n).await {
                Ok(val) => {
                    ::std::io::_print(format_args!("{0}! = {1:?}\n", n, val));
                }
                Err(e) => {
                    ::std::io::_print(
                        format_args!("Error calculating {0}!: {1}\n", n, e),
                    );
                }
            }
        }
        Ok(())
    }
}
mod server {
    use crate::service::{MathService, MathServiceServer};
    use tarpc::context;
    use tarpc::tokio_serde::formats::Json;
    use tokio::net::TcpListener;
    struct MathServer;
    #[automatically_derived]
    impl ::core::clone::Clone for MathServer {
        #[inline]
        fn clone(&self) -> MathServer {
            MathServer
        }
    }
    impl MathService for MathServer {
        fn add(
            self,
            _context: context::Context,
            a: i32,
            b: i32,
        ) -> ::core::pin::Pin<
            Box<dyn ::core::future::Future<Output = i32> + ::core::marker::Send>,
        > {
            Box::pin(async move { a + b })
        }
        fn factorial(
            self,
            _context: context::Context,
            n: u64,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<u64, String>,
                > + ::core::marker::Send,
            >,
        > {
            Box::pin(async move {
                if n > 20 {
                    return Err("Number too large!".to_string());
                }
                Ok((1..=n).product())
            })
        }
        type AddFut = ::core::pin::Pin<
            Box<dyn ::core::future::Future<Output = i32> + ::core::marker::Send>,
        >;
        type FactorialFut = ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<u64, String>,
                > + ::core::marker::Send,
            >,
        >;
    }
    pub async fn run_server(listener: TcpListener) -> anyhow::Result<()> {
        {
            ::std::io::_print(
                format_args!("Server running on {0}\n", listener.local_addr()?),
            );
        };
        loop {
            let (socket, addr) = listener.accept().await?;
            {
                ::std::io::_print(format_args!("New client connected: {0}\n", addr));
            };
            let transport = tarpc::serde_transport::new(
                tarpc::tokio_util::codec::length_delimited::LengthDelimitedCodec::builder()
                    .new_framed(socket),
                Json::default(),
            );
            let server = MathServiceServer::new(MathServer);
            tokio::spawn(async move {
                if let Err(e) = server.serve(transport).await {
                    {
                        ::std::io::_eprint(format_args!("Server error: {0}\n", e));
                    };
                }
            });
        }
    }
}
mod service {
    use tarpc::context;
    pub trait MathService: Sized {
        ///The response future returned by [`MathService::add`].
        type AddFut: std::future::Future<Output = i32>;
        fn add(self, context: tarpc::context::Context, a: i32, b: i32) -> Self::AddFut;
        ///The response future returned by [`MathService::factorial`].
        type FactorialFut: std::future::Future<Output = Result<u64, String>>;
        fn factorial(
            self,
            context: tarpc::context::Context,
            n: u64,
        ) -> Self::FactorialFut;
        /// Returns a serving function to use with
        /// [InFlightRequest::execute](tarpc::server::InFlightRequest::execute).
        fn serve(self) -> ServeMathService<Self> {
            ServeMathService { service: self }
        }
    }
    /// A serving function to use with [tarpc::server::InFlightRequest::execute].
    pub struct ServeMathService<S> {
        service: S,
    }
    #[automatically_derived]
    impl<S: ::core::clone::Clone> ::core::clone::Clone for ServeMathService<S> {
        #[inline]
        fn clone(&self) -> ServeMathService<S> {
            ServeMathService {
                service: ::core::clone::Clone::clone(&self.service),
            }
        }
    }
    impl<S> tarpc::server::Serve<MathServiceRequest> for ServeMathService<S>
    where
        S: MathService,
    {
        type Resp = MathServiceResponse;
        type Fut = MathServiceResponseFut<S>;
        fn method(&self, req: &MathServiceRequest) -> Option<&'static str> {
            Some(
                match req {
                    MathServiceRequest::Add { .. } => "MathService.add",
                    MathServiceRequest::Factorial { .. } => "MathService.factorial",
                },
            )
        }
        fn serve(
            self,
            ctx: tarpc::context::Context,
            req: MathServiceRequest,
        ) -> Self::Fut {
            match req {
                MathServiceRequest::Add { a, b } => {
                    MathServiceResponseFut::Add(
                        MathService::add(self.service, ctx, a, b),
                    )
                }
                MathServiceRequest::Factorial { n } => {
                    MathServiceResponseFut::Factorial(
                        MathService::factorial(self.service, ctx, n),
                    )
                }
            }
        }
    }
    /// The request sent over the wire from the client to the server.
    #[allow(missing_docs)]
    #[serde(crate = "tarpc::serde")]
    pub enum MathServiceRequest {
        Add { a: i32, b: i32 },
        Factorial { n: u64 },
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        use tarpc::serde as _serde;
        #[automatically_derived]
        impl tarpc::serde::Serialize for MathServiceRequest {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> tarpc::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: tarpc::serde::Serializer,
            {
                match *self {
                    MathServiceRequest::Add { ref a, ref b } => {
                        let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "MathServiceRequest",
                            0u32,
                            "Add",
                            0 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "a",
                            a,
                        )?;
                        _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "b",
                            b,
                        )?;
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                    MathServiceRequest::Factorial { ref n } => {
                        let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "MathServiceRequest",
                            1u32,
                            "Factorial",
                            0 + 1,
                        )?;
                        _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "n",
                            n,
                        )?;
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        use tarpc::serde as _serde;
        #[automatically_derived]
        impl<'de> tarpc::serde::Deserialize<'de> for MathServiceRequest {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> tarpc::serde::__private::Result<Self, __D::Error>
            where
                __D: tarpc::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 2",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Add" => _serde::__private::Ok(__Field::__field0),
                            "Factorial" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Add" => _serde::__private::Ok(__Field::__field0),
                            b"Factorial" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<MathServiceRequest>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MathServiceRequest;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum MathServiceRequest",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            1u64 => _serde::__private::Ok(__Field::__field1),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "a" => _serde::__private::Ok(__Field::__field0),
                                            "b" => _serde::__private::Ok(__Field::__field1),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"a" => _serde::__private::Ok(__Field::__field0),
                                            b"b" => _serde::__private::Ok(__Field::__field1),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<MathServiceRequest>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = MathServiceRequest;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant MathServiceRequest::Add",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            i32,
                                        >(&mut __seq)? {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant MathServiceRequest::Add with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match _serde::de::SeqAccess::next_element::<
                                            i32,
                                        >(&mut __seq)? {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant MathServiceRequest::Add with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(MathServiceRequest::Add {
                                            a: __field0,
                                            b: __field1,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                                        let mut __field1: _serde::__private::Option<i32> = _serde::__private::None;
                                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("a"),
                                                        );
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private::Option::is_some(&__field1) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("b"),
                                                        );
                                                    }
                                                    __field1 = _serde::__private::Some(
                                                        _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                _serde::__private::de::missing_field("a")?
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private::Some(__field1) => __field1,
                                            _serde::__private::None => {
                                                _serde::__private::de::missing_field("b")?
                                            }
                                        };
                                        _serde::__private::Ok(MathServiceRequest::Add {
                                            a: __field0,
                                            b: __field1,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &["a", "b"];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<
                                            MathServiceRequest,
                                        >,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                            (__Field::__field1, __variant) => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "n" => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"n" => _serde::__private::Ok(__Field::__field0),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<MathServiceRequest>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = MathServiceRequest;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant MathServiceRequest::Factorial",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            u64,
                                        >(&mut __seq)? {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant MathServiceRequest::Factorial with 1 element",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(MathServiceRequest::Factorial {
                                            n: __field0,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<u64> = _serde::__private::None;
                                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("n"),
                                                        );
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        _serde::de::MapAccess::next_value::<u64>(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                _serde::__private::de::missing_field("n")?
                                            }
                                        };
                                        _serde::__private::Ok(MathServiceRequest::Factorial {
                                            n: __field0,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &["n"];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<
                                            MathServiceRequest,
                                        >,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["Add", "Factorial"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "MathServiceRequest",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<MathServiceRequest>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(missing_docs)]
    impl ::core::fmt::Debug for MathServiceRequest {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                MathServiceRequest::Add { a: __self_0, b: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Add",
                        "a",
                        __self_0,
                        "b",
                        &__self_1,
                    )
                }
                MathServiceRequest::Factorial { n: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Factorial",
                        "n",
                        &__self_0,
                    )
                }
            }
        }
    }
    /// The response sent over the wire from the server to the client.
    #[allow(missing_docs)]
    #[serde(crate = "tarpc::serde")]
    pub enum MathServiceResponse {
        Add(i32),
        Factorial(Result<u64, String>),
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        use tarpc::serde as _serde;
        #[automatically_derived]
        impl tarpc::serde::Serialize for MathServiceResponse {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> tarpc::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: tarpc::serde::Serializer,
            {
                match *self {
                    MathServiceResponse::Add(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "MathServiceResponse",
                            0u32,
                            "Add",
                            __field0,
                        )
                    }
                    MathServiceResponse::Factorial(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "MathServiceResponse",
                            1u32,
                            "Factorial",
                            __field0,
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        use tarpc::serde as _serde;
        #[automatically_derived]
        impl<'de> tarpc::serde::Deserialize<'de> for MathServiceResponse {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> tarpc::serde::__private::Result<Self, __D::Error>
            where
                __D: tarpc::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 2",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Add" => _serde::__private::Ok(__Field::__field0),
                            "Factorial" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Add" => _serde::__private::Ok(__Field::__field0),
                            b"Factorial" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<MathServiceResponse>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MathServiceResponse;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum MathServiceResponse",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::__private::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<
                                        i32,
                                    >(__variant),
                                    MathServiceResponse::Add,
                                )
                            }
                            (__Field::__field1, __variant) => {
                                _serde::__private::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<
                                        Result<u64, String>,
                                    >(__variant),
                                    MathServiceResponse::Factorial,
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["Add", "Factorial"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "MathServiceResponse",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<MathServiceResponse>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(missing_docs)]
    impl ::core::fmt::Debug for MathServiceResponse {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                MathServiceResponse::Add(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Add",
                        &__self_0,
                    )
                }
                MathServiceResponse::Factorial(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Factorial",
                        &__self_0,
                    )
                }
            }
        }
    }
    /// A future resolving to a server response.
    #[allow(missing_docs)]
    pub enum MathServiceResponseFut<S: MathService> {
        Add(<S as MathService>::AddFut),
        Factorial(<S as MathService>::FactorialFut),
    }
    impl<S: MathService> std::fmt::Debug for MathServiceResponseFut<S> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.debug_struct("MathServiceResponseFut").finish()
        }
    }
    impl<S: MathService> std::future::Future for MathServiceResponseFut<S> {
        type Output = MathServiceResponse;
        fn poll(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<MathServiceResponse> {
            unsafe {
                match std::pin::Pin::get_unchecked_mut(self) {
                    MathServiceResponseFut::Add(resp) => {
                        std::pin::Pin::new_unchecked(resp)
                            .poll(cx)
                            .map(MathServiceResponse::Add)
                    }
                    MathServiceResponseFut::Factorial(resp) => {
                        std::pin::Pin::new_unchecked(resp)
                            .poll(cx)
                            .map(MathServiceResponse::Factorial)
                    }
                }
            }
        }
    }
    #[allow(unused)]
    /// The client stub that makes RPC calls to the server. All request methods return
    /// [Futures](std::future::Future).
    pub struct MathServiceClient(
        tarpc::client::Channel<MathServiceRequest, MathServiceResponse>,
    );
    #[automatically_derived]
    #[allow(unused)]
    impl ::core::clone::Clone for MathServiceClient {
        #[inline]
        fn clone(&self) -> MathServiceClient {
            MathServiceClient(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    #[allow(unused)]
    impl ::core::fmt::Debug for MathServiceClient {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "MathServiceClient",
                &&self.0,
            )
        }
    }
    impl MathServiceClient {
        /// Returns a new client stub that sends requests over the given transport.
        pub fn new<T>(
            config: tarpc::client::Config,
            transport: T,
        ) -> tarpc::client::NewClient<
            Self,
            tarpc::client::RequestDispatch<MathServiceRequest, MathServiceResponse, T>,
        >
        where
            T: tarpc::Transport<
                tarpc::ClientMessage<MathServiceRequest>,
                tarpc::Response<MathServiceResponse>,
            >,
        {
            let new_client = tarpc::client::new(config, transport);
            tarpc::client::NewClient {
                client: MathServiceClient(new_client.client),
                dispatch: new_client.dispatch,
            }
        }
    }
    impl MathServiceClient {
        #[allow(unused)]
        pub fn add(
            &self,
            ctx: tarpc::context::Context,
            a: i32,
            b: i32,
        ) -> impl std::future::Future<
            Output = Result<i32, tarpc::client::RpcError>,
        > + '_ {
            let request = MathServiceRequest::Add { a, b };
            let resp = self.0.call(ctx, "MathService.add", request);
            async move {
                match resp.await? {
                    MathServiceResponse::Add(msg) => std::result::Result::Ok(msg),
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
        #[allow(unused)]
        pub fn factorial(
            &self,
            ctx: tarpc::context::Context,
            n: u64,
        ) -> impl std::future::Future<
            Output = Result<Result<u64, String>, tarpc::client::RpcError>,
        > + '_ {
            let request = MathServiceRequest::Factorial { n };
            let resp = self.0.call(ctx, "MathService.factorial", request);
            async move {
                match resp.await? {
                    MathServiceResponse::Factorial(msg) => std::result::Result::Ok(msg),
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
    }
}
/// Tarpc 示例程序
#[command(version, about, long_about = None)]
struct Args {
    /// 运行模式 (server/client)
    #[arg(short, long, default_value = "server")]
    mode: String,
    /// 服务器地址 (仅客户端需要)
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    addr: String,
}
#[automatically_derived]
#[allow(unused_qualifications, clippy::redundant_locals)]
impl clap::Parser for Args {}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::CommandFactory for Args {
    fn command<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("rpc_servcie");
        <Self as clap::Args>::augment_args(__clap_app)
    }
    fn command_for_update<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("rpc_servcie");
        <Self as clap::Args>::augment_args_for_update(__clap_app)
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::FromArgMatches for Args {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = Args {
            mode: __clap_arg_matches
                .remove_one::<String>("mode")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: mode",
                ))?,
            addr: __clap_arg_matches
                .remove_one::<String>("addr")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: addr",
                ))?,
        };
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if __clap_arg_matches.contains_id("mode") {
            #[allow(non_snake_case)]
            let mode = &mut self.mode;
            *mode = __clap_arg_matches
                .remove_one::<String>("mode")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: mode",
                ))?;
        }
        if __clap_arg_matches.contains_id("addr") {
            #[allow(non_snake_case)]
            let addr = &mut self.addr;
            *addr = __clap_arg_matches
                .remove_one::<String>("addr")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: addr",
                ))?;
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::Args for Args {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("Args"))
    }
    fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("Args")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 2usize] = [
                                clap::Id::from("mode"),
                                clap::Id::from("addr"),
                            ];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("mode")
                        .value_name("MODE")
                        .required(false && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::impl_prelude::*;
                            let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                String,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .help("运行模式 (server/client)")
                        .long_help(None)
                        .short('m')
                        .long("mode")
                        .default_value("server");
                    let arg = arg;
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("addr")
                        .value_name("ADDR")
                        .required(false && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::impl_prelude::*;
                            let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                String,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .help("服务器地址 (仅客户端需要)")
                        .long_help(None)
                        .short('a')
                        .long("addr")
                        .default_value("127.0.0.1:8080");
                    let arg = arg;
                    arg
                });
            __clap_app
                .about("Tarpc 示例程序")
                .long_about(None)
                .version("0.1.0")
                .long_about(None)
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("Args")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 2usize] = [
                                clap::Id::from("mode"),
                                clap::Id::from("addr"),
                            ];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("mode")
                        .value_name("MODE")
                        .required(false && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::impl_prelude::*;
                            let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                String,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .help("运行模式 (server/client)")
                        .long_help(None)
                        .short('m')
                        .long("mode")
                        .default_value("server");
                    let arg = arg.required(false);
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("addr")
                        .value_name("ADDR")
                        .required(false && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::impl_prelude::*;
                            let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                String,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .help("服务器地址 (仅客户端需要)")
                        .long_help(None)
                        .short('a')
                        .long("addr")
                        .default_value("127.0.0.1:8080");
                    let arg = arg.required(false);
                    arg
                });
            __clap_app
                .about("Tarpc 示例程序")
                .long_about(None)
                .version("0.1.0")
                .long_about(None)
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Args {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Args",
            "mode",
            &self.mode,
            "addr",
            &&self.addr,
        )
    }
}
fn main() -> anyhow::Result<()> {
    let body = async {
        let args = Args::parse();
        match args.mode.to_lowercase().as_str() {
            "server" => {
                {
                    ::std::io::_print(
                        format_args!("Starting server on {0}\n", args.addr),
                    );
                };
                let listener = TcpListener::bind(&args.addr).await?;
                server::run_server(listener).await
            }
            "client" => {
                {
                    ::std::io::_print(
                        format_args!("Connecting to server at {0}\n", args.addr),
                    );
                };
                client::run_client(&args.addr).await
            }
            _ => {
                {
                    ::std::io::_eprint(
                        format_args!("Invalid mode. Use \'server\' or \'client\'.\n"),
                    );
                };
                std::process::exit(1);
            }
        }
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return
    )]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}

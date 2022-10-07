macro_rules! endpoint_common {
    (I $query:ty, $body:ty) => {
        fn _method(&self) -> Method {
            self.method()
        }

        fn _path(&self) -> String {
            self.path()
        }

        fn _query(&self) -> Option<$query> {
            self.query()
        }

        fn _body(&self) -> Option<$body> {
            self.body()
        }

        fn _headers(&self) -> Option<Vec<(String, String)>> {
            self.headers()
        }
    };
    (P $query:ty, $body:ty) => {
        fn method(&self) -> Method {
            Method::GET
        }

        fn path(&self) -> String {
            "".to_string()
        }

        fn query(&self) -> Option<QueryType> {
            None
        }

        fn body(&self) -> Option<BodyType> {
            None
        }

        fn headers(&self) -> Option<Vec<(String, String)>> {
            None
        }
    };
}

pub(super) use endpoint_common;

macro_rules! endpoint {
    (P $name:ident, $result:ty, { [$resp:ident, $self:ident] $($body:tt)* }) => {
        endpoint!(P internal, $name, $result, { [$resp, $self] $($body)* });
    };
    (T $name:ident, $result:ident, { [$resp:ident, $self:ident] $($body:tt)* }) => {
        endpoint!(T internal, $name, $result, { [$resp, $self] $($body)* });
    };
    (P $namespace:ident, $name:ident, $result:ty, { [$resp:ident, $self:ident] $($body:tt)* }) => {
        mod $namespace {
            use crate::{
                api::{
                    endpoint::Endpoint,
                    types::{QueryParams, RequestBody},
                },
            };
            use anyhow::Result;
            use async_trait::async_trait;
            use hyper::{Body, Response, Method};
            use $crate::api::endpoint::macros::endpoint_common;

            pub trait $name<QueryType, BodyType>: Sync + Send
            where
                QueryType: QueryParams,
                BodyType: RequestBody,
            {
                endpoint_common!(P QueryType, BodyType);

                fn to_endpoint(self) -> Box<dyn Endpoint<$result, QueryType, BodyType>>
                where
                    Self: Sized + 'static,
                    QueryType: QueryParams + 'static,
                    BodyType: RequestBody + 'static,
                {
                    Box::new(Box::new(self) as Box<dyn $name<QueryType, BodyType>>)
                }
            }

            #[async_trait]
            impl<QueryType: QueryParams, BodyType: RequestBody> Endpoint<$result, QueryType, BodyType>
                for Box<dyn $name<QueryType, BodyType>>
            {
                endpoint_common!(I QueryType, BodyType);

                async fn _result(&$self, $resp: Response<Body>) -> Result<$result> {
                    $($body)*
                }
            }
        }

        pub use $namespace::$name;
    };
    (T $namespace:ident, $name:ident, $result:ident, { [$resp:ident, $self:ident] $($body:tt)* }) => {
        mod $namespace {
            use crate::{
                api::{
                    endpoint::Endpoint,
                    types::{QueryParams, RequestBody},
                },
            };
            use anyhow::Result;
            use async_trait::async_trait;
            use hyper::{Body, Response, Method};
            use $crate::api::endpoint::macros::endpoint_common;
            use super::*;

            pub trait $name<ResultType, QueryType, BodyType>: Sync + Send
            where
                ResultType: $result,
                QueryType: QueryParams,
                BodyType: RequestBody,
            {
                endpoint_common!(P QueryType, BodyType);

                fn to_endpoint(self) -> Box<dyn Endpoint<ResultType, QueryType, BodyType>>
                where
                    Self: Sized + 'static,
                    ResultType: $result + 'static,
                    QueryType: QueryParams + 'static,
                    BodyType: RequestBody + 'static,
                {
                    Box::new(Box::new(self) as Box<dyn $name<ResultType, QueryType, BodyType>>)
                }
            }

            #[async_trait]
            impl<ResultType: $result, QueryType: QueryParams, BodyType: RequestBody> Endpoint<ResultType, QueryType, BodyType>
                for Box<dyn $name<ResultType, QueryType, BodyType>>
            {
                endpoint_common!(I QueryType, BodyType);

                async fn _result(&$self, $resp: Response<Body>) -> Result<ResultType> {
                    $($body)*
                }
            }
        }
        
        pub use $namespace::$name;
    };
}

pub(super) use endpoint;

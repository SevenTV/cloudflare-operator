macro_rules! endpoint {
    (P $name:ident, $result:ty, { [$resp:ident, $self:ident] $($body:tt)* }) => {
        const _: () = {
            use async_trait::async_trait;
            use serde::Serialize;
            use hyper::{Body, Response, Method};
            use super::$name;
            use $crate::{api::{endpoint::Endpoint, types::RequestBody}, types::Result};

            #[async_trait]
            impl<QueryType: Serialize, BodyType: RequestBody> Endpoint<$result, QueryType, BodyType>
                for Box<dyn $name<QueryType, BodyType>>
            {
                fn _method(&self) -> Method {
                    self.method()
                }

                fn _path(&self) -> String {
                    self.path()
                }

                fn _query(&self) -> Option<QueryType> {
                    self.query()
                }

                fn _body(&self) -> Option<BodyType> {
                    self.body()
                }

                fn _headers(&self) -> Option<Vec<(String, String)>> {
                    self.headers()
                }

                async fn _result(&$self, $resp: Response<Body>) -> Result<$result> {
                    $($body)*
                }
            }
        };
    };
    (T $name:ident, $result:ident, { [$resp:ident, $self:ident] $($body:tt)* }) => {
        const _: () = {
            use async_trait::async_trait;
            use serde::Serialize;
            use hyper::{Body, Response, Method};
            use super::$name;
            use $crate::{api::{endpoint::Endpoint, types::RequestBody}, types::Result};

            #[async_trait]
            impl<ResultType: $result, QueryType: Serialize, BodyType: RequestBody> Endpoint<ResultType, QueryType, BodyType>
                for Box<dyn $name<ResultType, QueryType, BodyType>>
            {
                fn _method(&self) -> Method {
                    self.method()
                }

                fn _path(&self) -> String {
                    self.path()
                }

                fn _query(&self) -> Option<QueryType> {
                    self.query()
                }

                fn _body(&self) -> Option<BodyType> {
                    self.body()
                }

                fn _headers(&self) -> Option<Vec<(String, String)>> {
                    self.headers()
                }

                async fn _result(&$self, $resp: Response<Body>) -> Result<ResultType> {
                    $($body)*
                }
            }
        };
    };
}

pub(super) use endpoint;

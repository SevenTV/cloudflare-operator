use super::macros::endpoint;

endpoint!(P StringEndpoint, String, { [ resp, self ]
    Ok(
        String::from_utf8_lossy(&hyper::body::to_bytes(resp.into_body()).await?.to_vec())
            .to_string(),
    )
});

pub use internal::StringEndpoint;

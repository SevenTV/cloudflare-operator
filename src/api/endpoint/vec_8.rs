use super::macros::endpoint;

endpoint!(P Vec8Endpoint, Vec<u8>, { [ resp, self ]
    Ok(
        hyper::body::to_bytes(resp.into_body()).await?.to_vec()
    )
});

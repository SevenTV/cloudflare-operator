# We need benchmarks for this

Ones I am interested in are:

- Requests per second on varying levels of concurrency.

- Memory overhead per connection. (excluding a read/write buffer purely the memory usage which is not controlled by the user)

- Fixed requests per second on high levels of concurrency, we open at a rate of x connections per second and we do not close them.

We also need to write unit tests for pretty much everything.

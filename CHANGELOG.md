# 0.9
Refactor the library away from having a single global client, but provide a client of our own that
the user of the library is responsible for. This means that the user has control over the allocation
and destruction of the client. This solves issue #60 and is enabled due to tireless work by
shepmaster. Big thanks!

# 0.10
Small fix to the public interface of `sync::ObjectClient` that was not properly sync.
Update cloud storage to use the new url, `www.googleapis.com` => `storage.googleapis.com`

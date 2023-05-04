# 1.0.0
Refactored the library to have one model per file, to improve maintainabillity.
Refactor the library to use less static variables (mainly ServiceAccount)
Added a ClientBuilder by @alexpusch
Made the unmaintained dependency `dotenv` optional
Provided a new way to load in ServiceAccount configuration: `ServiceAccount::from_str()`
Dramatically improved download performance by streaming an array of bytes, rather than a single byte per poll
Moved variables used by all functions in a client to the constructor of the client, most commonly the bucket
Replaced `chrono` with `time` by @Elykz
Added optional QueryParameters to be sent along with the requests by @SergenN
Added missing GCP locations by @trigovision

# 0.9
Refactor the library away from having a single global client, but provide a client of our own that
the user of the library is responsible for. This means that the user has control over the allocation
and destruction of the client. This solves issue #60 and is enabled due to tireless work by
shepmaster. Big thanks!

# 0.10
Small fix to the public interface of `sync::ObjectClient` that was not properly sync.
Fix urlencoding url paths correctly in several places.
Update cloud storage to use the new url, `www.googleapis.com` => `storage.googleapis.com`

# 0.11
@pseguin2011: Implemented a configurable authentication layer through the `TokenCache` trait.

# 0.12
Implement customisable authentication providers, via the `Client::with_cache` method.

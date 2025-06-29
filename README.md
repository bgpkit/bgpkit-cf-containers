# BGP Data Search API with Cloudflare Containers

[Cloudflare Containers][containers] provides a platform to run containerized applications.
The platform is well-suited for short-lived bursts of queries.
This repository contains a simple example of how to deploy a Rust-based BGP Data Search API using Cloudflare Containers.

[containers]: https://developers.cloudflare.com/containers/

## BGP Data Search API

The BGP data search API allows searching for BGP updates data archived on RouteViews or RIPE RIS
and is built with BGPKIT in Rust.
Using the `axum` web framework, it provides a simple interface to query BGP data.
The API provides the `/search` endpoint with the following required parameters:

- `collector`: The BGP collector to query, e.g., `ris`, `routeviews`, etc.
- `prefix`: The BGP prefix to search for, e.g., `1.1.1.0/24`.
- `ts_start`: The start time of the query, RFC3339 format, e.g., `2023-01-01T00:00:00Z` or Unix timestamp in seconds.
- `ts_end`: The end time of the query, RFC3339 format, e.g., `2023-01-02T00:00:00Z` or Unix timestamp in seconds.

In the backend, it searches relevant BGP data files with `bgpkit_broker` and parses the found BGP updates data using `bgpkit_parser`.

It returns the parsed BGP updates data in JSON format.

## Cloudflare Workers API

We also need to run a wrapper API to handle the HTTP requests and route them to the containerized BGP Data Search API.

## Build and deploy

The project utilizes the Cloudflare Containers tooling to build and deploy the application.
To build and deploy the application, run the following command:
```
npx wrangler deploy
```
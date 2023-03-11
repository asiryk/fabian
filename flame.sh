#!/usr/bin/env bash

CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --bin benchmark

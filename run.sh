#!/bin/bash
USERNAME=$(jq -r '.username' /data/options.json)
/build/target/release/test_addon

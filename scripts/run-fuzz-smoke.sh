#!/usr/bin/env bash
set -euo pipefail
cd harnesses && cargo fuzz run ownable -- -max_total_time=10

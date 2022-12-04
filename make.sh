#!/bin/bash

set -eu

cargo generate --path template --vcs none --name $1

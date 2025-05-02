#!/bin/sh
set -e
cd services/rest-fights
./deploy.sh
cd ../..
cd services/rest-heroes
./deploy.sh
cd ../..


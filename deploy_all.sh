#!/bin/sh
set -e
cd services/rest-heroes
./deploy.sh
cd ../..
cd services/rest-locations
./deploy.sh
cd ../..
cd services/rest-fights
./deploy.sh
cd ../..


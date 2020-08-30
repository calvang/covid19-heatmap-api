#!/bin/bash
# Script to get updated COVID-19 global from the COVID-19 API
# Schedule a cron job to automate these updates

cd src/dataset

echo "Retrieving current global data..."
curl https://api.covid19api.com/summary > currentGlobalData.json

#!/bin/bash
# Script to get updated Covid-19 data on counties and process it to be readable by the map
# Schedule a cron job to automate these updates

echo "Retrieving updated county data..."
curl https://usafactsstatic.blob.core.windows.net/public/data/covid-19/covid_confirmed_usafacts.csv > currentCountyCases.csv

echo "Retrieving updated Brazilian state data..."
curl https://static01.nyt.com/newsgraphics/2020/03/16/coronavirus-maps/1edc90f6c2c7ab866ebecd97bcfaa01086ff3557/data/timeseries/en/BRA.json > brazilstates.json

echo "Processing data..."
python3 processCounties.py
python3 processCountiesShort.py
python3 processBrazil.py
python3 processBrazilShort.py

echo "Done."
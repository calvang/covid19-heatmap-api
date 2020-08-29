import csv
import json

# merge census data from worldbank.org with json data
# 1. get population data and ISO3 identified from countryPopulations.csv
# 2. get ISO3 and ISO2 from countryInfo.json
# 3. get current full data
# 4. output full data with appended ISO3 and population elements

populationList = []

# read in csv file
with open('countryPopulations.csv') as csv_file:
    csv_reader = csv.reader(csv_file, delimiter=',')
    line_count = 0
    for row in csv_reader:
        if line_count < 5:
            print(f'Column names are {", ".join(row)}')
            line_count += 1
        else:
            print(len(row))
            setPop = {}
            setPop['ISO3'] = row[1]

            max_index = 4
            index = 0
            for i in row:
                if  index >= max_index and i != '':
                    max_index = index
                index += 1
            setPop['Population'] = row[max_index]
            print("Set: " + setPop['ISO3'] + ", " + setPop['Population'])
            populationList.append(setPop)
            line_count += 1
    print(f'Processed {len(populationList)} lines.')

countryInfo = []

# data to map ISO3 to ISO2
with open('countryInfo.json', 'r') as myfile1:
    data1=myfile1.read()
    countryInfo = json.loads(data1)

fullData = []

# current full dataset
with open('countryDataFull.json', 'r') as myfile2:
    data2=myfile2.read()
    fullData = json.loads(data2)

# store ISO2 values in populationList
for i in populationList:
    for j in countryInfo:
        if i['ISO3'] == j['iso3']:
            i['ISO2'] = j['iso2']
            # Get emojis too!
            i['Emoji'] = j['emoji']
            print("New ISO2: " + i['ISO2'])
            break
print("Initial: " + fullData[0]['ISO2'])
# store ISO3 and population in full data
for i in fullData:
    for j in populationList:
        if 'ISO2' in j and 'ISO2' in i and i['ISO2'] == j['ISO2']:
            i['ISO3'] = j['ISO3']
            i['Emoji'] = j['Emoji']
            i['Population'] = j['Population']
            print("New Pop: " + i['Population'])
            break

with open('fullDataSet.json', 'w') as outfile:
    outfile.write(json.dumps(fullData, ensure_ascii=False))


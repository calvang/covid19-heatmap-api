import csv, json

abridged_data = []

with open('brazilstates.json','r') as file1:
    data = file1.read()
    brazil_data = json.loads(data)
    states_data = brazil_data['data']

for index, state in enumerate(states_data):
    if index != 0:
        state_dict = {
            'State': state['display_name'],
            'Deaths': int(state['deaths'][len(state['deaths'])-1]),
            'Confirmed': int(state['latest']),
            'Recovered': int(state['recovered'][len(state['recovered'])-1]),
            'Population': int(state['population'])
        }
        abridged_data.append(state_dict)

with open('brazilstatescoords.csv') as file2:
    reader = csv.reader(file2, delimiter=',')
    print(len(abridged_data))
    for index, row in enumerate(reader):
        if index != 0:
            print(index)
            abridged_data[index-1]['lat'] = float(str(row[1]).strip())
            abridged_data[index-1]['lon'] = float(str(row[2]).strip())

with open('brazilStateDataFull.json', 'w') as outfile:
    outfile.write(json.dumps(abridged_data, indent=4, sort_keys=True, ensure_ascii=False))

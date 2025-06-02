# link for reference https://docs.propellerheads.xyz/tycho/for-solvers/indexer/tycho-rpc#v1-tokens

from urllib import response
import requests
import json

# chains = ['ethereum', 'starknet', 'zksync', 'arbitrum', 'base', 'unichain']

headers = {
    "Content-Type": "application/json",
    "Authorization": "sampletoken"
}

# Stores the to be JSOn dict
dict_store = {}

# # Retreive the contracts in each system
# con_state_url = "https://tycho-beta.propellerheads.xyz/v1/contract_state"
# con_state_data = {
#     "chain": "ethereum"
# }
# response = requests.post(con_state_url, headers=headers, json=con_state_data)
# dict_store['contracts'] = response.json()['accounts']

# Retreive the system URL (uniswap, pancake swap etc)
proc_system_url = "https://tycho-beta.propellerheads.xyz/v1/protocol_systems"
proc_system_data = {
    "chain": "ethereum"
}
response = requests.post(proc_system_url, headers=headers, json=proc_system_data)

print("Status Code:", response.status_code)
print("Response Body:", response.json()['protocol_systems'])

proc_sys = response.json()['protocol_systems']

# Retive the state including the liquidity of each system 
proc_state_url = "https://tycho-beta.propellerheads.xyz/v1/protocol_state"

for system in proc_sys:
    try:
        data = {
            "protocol_system": system
        }
        response = requests.post(proc_state_url, headers=headers, json=data)

        print(f"Status Code (Protocol State for {system}):", response.status_code)
        try:
            response_json = response.json()
            tba = []
            for state in response_json['states']:
                if not ('liquidity' in state['attributes'] and state['attributes']['liquidity'] == '0x00'):
                    tba.append(state)
            dict_store[system] = tba
            print(f"Response Body (Protocol State for {system}) succesfully parsed")
            
        except json.JSONDecodeError:
            print(f"Error: Could not parse Protocol State response for {system} as JSON. Response text: {response.text}")
            
    except requests.exceptions.RequestException as e:
        print(f"Error making request for protocol state of {system}: {e}")


# Convert the dictionary to a JSON string
json_string = json.dumps(dict_store, indent=4)

# Write the JSON string to a file
file_path = "../ingestor/liquidity_data.json"

try:
    with open(file_path, "w") as f:
        f.write(json_string)
    print(f"JSON data written to {file_path}")
except Exception as e:
    print(f"An error occurred while writing to the file: {e}")
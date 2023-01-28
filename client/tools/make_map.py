import json
import random

def main():
    res = {
        "rock": 3,
        "sheep": 4,
        "wood": 4,
        "clay": 3,
        "hay": 4,
        "": 1
    }
    res_list = []
    stat_list = []
    # Value
    for e in res:
        for _ in range(res[e]):
            res_list.append(e)
    random.shuffle(res_list)
    # Value
    for val in range(2, 13):
        if val == 2:
            stat_list.append(val)
        elif val == 12:
            stat_list.append(val)
        elif val != 7:
            stat_list.append(val)
            stat_list.append(val)
    random.shuffle(stat_list)
    with open("map.json", "w+") as file:
        json.dump({"tile": res_list, "value": stat_list}, file, indent=2)
    print("Map done")
    

if __name__ == "__main__" :
    main()
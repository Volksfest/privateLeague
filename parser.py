#!/usr/bin/python

import sc2reader
import math
import json
import sys

if len(sys.argv) <2:
    print("Nee dpath to replay")
    exit()
path = sys.argv[1]
replay = sc2reader.load_replay(path)


obj = {}
obj["duration"] = {"min": math.floor(replay.game_length.seconds/60), "sec": replay.game_length.seconds%60}
obj["map_name"] = replay.map_name

obj["players"] = []

for p in replay.players:
    obj["players"].append(
        {"name": p.name,
         "color": {"name": p.color.name, "hex": p.color.hex},
         "race": p.play_race,
         "random": p.play_race != p.pick_race,
         "win": p.result == "Win"
            }
        )

obj["observers"] = []

for o in replay.observers:
    obj["observers"].append(o.name)

date = replay.date
obj["date"] = {
        "day": date.day,
        "month": date.month,
        "year": date.year,
        "hour": date.hour,
        "minute": date.minute,
        "second": date.second
        }

json = json.dumps(obj, sort_keys=True, indent=4)
print(json)

#file = open("summary.json", "w")
#file.write(json)
#file.write("\n")
#file.close()


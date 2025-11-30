import csv
import json
import unicodedata
import re

input_file = "src/data/draft_points.csv"
output_file = "src/data/draft_points.json"

formes = {
    "mega": "mega",
    "paldean": "paldea",
    "alolan": "alola",
    "galarian": "galar",
    "hisuian": "hisui",
    "female": "f",
    "male": "m",
    "incarnate": "",
    "midday": "",
}

# y-x megas, tauros-*-paldea


def name_to_id(name):
    base_name = name.strip().replace(" ", "").lower()
    start_forme = next((f for f in formes.keys() if base_name.startswith(f)), "")
    if start_forme != "":
        base_name = base_name[len(start_forme) :]
        base_name = base_name + formes[start_forme]
    else:
        end_forme = next((f for f in formes.keys() if base_name.endswith(f)), "")
        if end_forme != "":
            base_name = base_name[: -len(end_forme)]
            base_name = base_name + formes[end_forme]

    base_name = unicodedata.normalize("NFD", base_name)
    base_name = "".join(ch for ch in base_name if unicodedata.category(ch) != "Mn")
    base_name = re.sub(r"[^a-z0-9]", "", base_name)
    return base_name


if __name__ == "__main__":
    data = {}

    with open(input_file, newline="", encoding="utf-8") as f:
        reader = csv.reader(f)
        headers = next(reader)  # first row, e.g. "19 Points", "18 Points", ...

        # Extract numeric point values from headers
        point_values = []
        for h in headers:
            num = h.split()[0]  # "19 Points" -> "19"
            point_values.append(num)
            data[num] = []  # initialize each tier list

        # Process rows
        for row in reader:
            for point, value in zip(point_values, row):
                if value:
                    data[point].append(name_to_id(value))

    # Write JSON
    with open(output_file, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4)

# Datamorph

Datamorph is a utiltiy cli tool that takes a csv file and converts it to json. This tool was created because a DB IDE can generally get you CSV of your data really quickly but tends to be slower and problematic when getting JSON of the same data especially at higher record counts.

# Instructions

Basic usage is providing a csv and getting a json of the same name in the same directory

```
cargo install datamorph

datamorph some.csv
# ./some.json

```

Flags

```
Output:    -o someOther.json  (provide a new name to output)
LowerCase: -l                 (set all the headers to lowercase)
Pretty:    -p                 (set the json to generate pretty)
```

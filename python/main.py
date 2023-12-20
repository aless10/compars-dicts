from compars_dicts import compare_dicts

dict1 = {"a": 1, "b": 2, "c": 3}
dict2 = {"a": 1, "b": 3, "d": 4}

result = compare_dicts(dict1, dict2)

print("Added keys:", result.added)
print("Removed keys:", result.removed)
print("Modified keys:", result.modified)

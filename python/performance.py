rs = """

from compars_dicts import compare_dicts
def rs():
    dict1 = {"a": 1, "b": 2, "c": 3}
    dict2 = {"a": 1, "b": 3, "d": 4}

    return compare_dicts(dict1, dict2)

rs()
"""

deep = """

from deepdiff import DeepDiff
def deep():
    dict1 = {"a": 1, "b": 2, "c": 3}
    dict2 = {"a": 1, "b": 3, "d": 4}

    return DeepDiff(dict1, dict2)

deep()
"""

# cProfile solution
import cProfile

cProfile.run(rs)
cProfile.run(deep)

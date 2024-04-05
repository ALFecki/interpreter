# E0001

def a(): return 1
def a(): return "a" # OK

test = {1: "2", 2: "3", 3: "5"}


test["2"]

1 # Warn

def f(): return "a"
f() # Warn

def f(): return None
f() # OK


test1: int = "2"

print(a())

def g(): return f()

def f(): return 1
# def f(): return "a" # E0001: Reassignment of a function referenced by other functions

# E0002

class C:
    def __init__(self): pass # OK
# class C:
#     def __init__(a): pass # ERR

# E0003

# class C:
#     __init__ = 1 # ERR

import numpy as np
list = np.array([])
b = np.array([])
with open(r"c:\users\kind_\desktop\programmingprojects\adventcodium\day1\input.txt") as inp:
  inplen = sum(1 for line in inp)
with open(r"c:\users\kind_\desktop\programmingprojects\adventcodium\day1\input.txt") as inp:
  i = 0
  while i <= inplen:
    i = i+1
    a = inp.readline()
    a = a.strip()
    if a == '':
      print("b:", b)
      c = sum(b)
      list = np.concatenate((list, [c]))
      b = []
      continue
    b = np.concatenate((b, [int(a)]))
print("max:", max(list))
print("list:", list)

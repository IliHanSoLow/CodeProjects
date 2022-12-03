import numpy as np
list = np.array([])
b = np.array([])
with open(r"C:\Users\Kind_\Desktop\ProgrammingProjects\Adventcodium\Day1\input.txt") as inp:
  inplen = sum(1 for line in inp)
with open(r"C:\Users\Kind_\Desktop\ProgrammingProjects\Adventcodium\Day1\input.txt") as inp:
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
print("list:", list)
print("max:", max(list))

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

secondmax=np.array(list)
secondmax=secondmax[secondmax!=max(secondmax)]

thirdmax=np.array(secondmax)
thirdmax=thirdmax[thirdmax!=max(thirdmax)]

fm=max(list)
sm=max(secondmax)
tm=max(thirdmax)

print("max:", fm)
print("2nd max:", sm)
print("3nd max:", tm)

top3=fm+sm+tm
print("top3:", top3)
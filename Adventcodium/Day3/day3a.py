bp=[] #backpack
h1=[] #half 1
h2=[] #half 2
prioletters=[]
prioscore=0

with open(r"c:\users\kind_\desktop\programmingprojects\adventcodium\day3\input.txt", 'r') as inp:
  bp=[line.strip().replace(' ', '') for line in inp]
  i=0

  for round in bp:
    bpl=len(round)
    float(bpl)
    h1=round[slice(0,bpl//2)]
    h2=round[slice(bpl//2,bpl)]

    for j in range(len(h1)):

      for q in range(len(h2)):

        if h1[j]==h2[q]:
          prioletters.append(h2[q])
          break

      else:
        continue
      break
    i=i+1

  print(prioletters)

prioscore = sum({'a':1, 'b':2,'c':3,'d':4,'e':5,'f':6,'g':7,'h':8,'i':9,'j':10,'k':11,'l':12,'m':13,'n':14,'o':15,'p':16,'q':17,'r':18,'s':19,'t':20,'u':21,'v':22,'w':23,'x':24,'y':25,'z':26,'A':27, 'B':28,'C':29,'D':30,'E':31,'F':32,'G':33,'H':34,'I':35,'J':36,'K':37,'L':38,'M':39,'N':40,'O':41,'P':42,'Q':43,'R':44,'S':45,'T':46,'U':47,'V':48,'W':49,'X':50,'Y':51,'Z':52}[round] for round in prioletters)
print(prioscore)
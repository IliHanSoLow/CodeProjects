bp=[]
e1=[]
e2=[]
e3=[]
eletter=[]
prioletters=[]

prioscore=0

with open(r"c:\users\kind_\desktop\programmingprojects\adventcodium\day3\input.txt", 'r') as inp:
  bp=[line.strip().replace(' ', '') for line in inp]

  for round in bp:
    if e1==[]:
      e1=round
    elif e2==[]:
      e2=round
    elif e3==[]:
      e3=round
    if e1!=[] and e2!=[] and e3!=[]:
      for j in range(len(e1)):

        for q in range(len(e2)):

          for p in range(len(e3)):
            if e1[j]==e2[q]==e3[p]:
              eall=e1[j]
              prioletters.append(eall)
              e1=[]
              e2=[]
              e3=[]
              break

          else:
            continue
          break

print(prioletters)

prioscore = sum({'a':1, 'b':2,'c':3,'d':4,'e':5,'f':6,'g':7,'h':8,'i':9,'j':10,'k':11,'l':12,'m':13,'n':14,'o':15,'p':16,'q':17,'r':18,'s':19,'t':20,'u':21,'v':22,'w':23,'x':24,'y':25,'z':26,'A':27, 'B':28,'C':29,'D':30,'E':31,'F':32,'G':33,'H':34,'I':35,'J':36,'K':37,'L':38,'M':39,'N':40,'O':41,'P':42,'Q':43,'R':44,'S':45,'T':46,'U':47,'V':48,'W':49,'X':50,'Y':51,'Z':52}[round] for round in prioletters)
print(prioscore)
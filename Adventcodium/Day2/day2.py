with open(r"c:\users\kind_\desktop\programmingprojects\adventcodium\day2\input.txt", 'r') as inp:
  i=0
  enemy=[]
  you=[]
  data = [line.strip().replace(' ', '') for line in inp]
  for round in data:
    a = round
    enemy.append(a[0])
    you.append(a[1])
    i=i+1
#A,X=Rock B,Y=Paper C,Z=Sissors
#A>C
#B>A
#C>B

  score=0

#C>B


  i=0
  for round in data:
    if you[i]=='X':
      score=score+1
        
    if you[i]=='Y':
      score=score+2

    if you[i]=='Z':
       score=score+3

    if you[i]=='X' and enemy[i]=='A':
      score=score+3

    if you[i]=='X' and enemy[i]=='C':
      score=score+6

    if you[i]=='Y' and enemy[i]=='B':
      score=score+3

    if you[i]=='Y' and enemy[i]=='A':
      score=score+6

    if you[i]=='Z' and enemy[i]=='C':
      score=score+3

    if you[i]=='Z' and enemy[i]=='B':
      score=score+6

    i=i+1

print(score)
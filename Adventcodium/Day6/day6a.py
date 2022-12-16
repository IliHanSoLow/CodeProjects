pasts=[]
x=0
location=0
counter=0
isStart=False

infile = r'C:\Users\Kind_\Desktop\ProgrammingProjects\Adventcodium\Day6\input.txt'
data = open(infile).readline()
for c in data:
  location+=1
  if len(pasts)!=4:
    pasts.append(c)
    print(pasts)
  else:
    pasts.append(c)
    pasts.pop(0)
    print(pasts)
    
    if pasts[0]!=pasts[1] and pasts[0]!=pasts[2] and pasts[0]!=pasts[3] and pasts[1]!=pasts[2] and pasts[1]!=pasts[3] and pasts[2] != pasts[3]:
      isStart=True

  if isStart:
    print(location)
    print(pasts)
    isStart=False
    exit()
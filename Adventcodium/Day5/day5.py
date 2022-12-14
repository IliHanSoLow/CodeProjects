infile = r'C:\Users\Kind_\Desktop\ProgrammingProjects\Adventcodium\Day5\inputb.txt'
data = open(infile).read().strip()
lines = [x.strip() for x in data.split('\n')]
# a1=['Z','N']
# a2=['M','C','D']
# a3=['P']
a1=['N','D','M','Q','B','P','Z']
a2=['C','L','Z','Q','M','D','H','V']
a3=['Q','H','R','D','V','F','Z','G']
a4=['H','G','D','F','N']
a5=['N','F','Q']
a6=['D','Q','V','Z','F','B','T']
a7=['Q','M','T','Z','D','V','S','H']
a8=['M','G','F','P','N','Q']
a9=['B','W','R','M']
for line in lines:
  one,two=line.split('from')
  s1=one.removeprefix('move ')
  s2,s3=two.removeprefix(' ').split(' to ')
  print(s3)

  for i in range(int(s1)):
    exec('a'+s3+'.append(a'+s2+'.pop())')

print(a1.pop(),a2.pop(),a3.pop(),a4.pop(),a5.pop(),a6.pop(),a7.pop(),a8.pop(),a9.pop())
counter=0
a=''
b=''
c=''
d=''
tmp=0
tmp2=False
with open(r"c:\users\kind_\desktop\programmingprojects\adventcodium\day4\input.txt", 'r') as inp:
  
  content=[line.strip().replace(' ','')for line in inp]
  
  for round in content:
    
    for i in round:
      if i==',' or i=='-':
        tmp=tmp+1
      elif tmp==0:
        a=a+i
      elif tmp==1:
        b=b+i
      elif tmp==2:
        c=c+i
      elif tmp==3:
        d=d+i


    # a=round[0]
    # if round[1]!='-':
      # a.append(round[1])
      # b=round[3]
      # if round[4]!=',':
        # b.append(round[4])
        # c=round[6]
        # if round[7]!='-':
          # b.append(round[7])


    # else:
      # b=round[2]
      # if round[3]!=',':
        # b.append(round[3])
    
    # for q in round:
      # if cp==True:
        # if q=='-':
          # dp=True
          # continue
        # c.append(q)
      # if q=='-':
        # bp=True
        # continue
      # if bp==True:
        # if bpp==True:
          # if q==',':
            # cp=True
            # b.append(q)
            # continue
        # b.append(q)
        # bpp=True
        # continue
      # a.append(q)

    print(a,b,c,d)

    for i in range(int(a),int(b)):
      if i==int(c):
          counter=counter+1
          break
      if i==int(d):
        counter=counter+1
        break
    for i in range(int(c),int(d)):
      if i==int(a):
        counter=counter+1
        break
      if i==int(b):
        counter=counter+1
        break
    a=''
    b=''
    c=''
    d=''
    tmp=0
  print(counter)
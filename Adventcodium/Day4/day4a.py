counter=0
counter2=0
a=[]
b=[]
c=[]
d=[]
tmp=0
# tmp2=False
with open(r"c:\users\kind_\desktop\programmingprojects\adventcodium\day4\input.txt", 'r') as inp:
  
  content=[line.strip().replace(' ','')for line in inp]
  
  for round in content:
    
    for i in round:
      if i==',' or i=='-':
        tmp=tmp+1
      elif tmp==0:
        a.append(int(i))
      elif tmp==1:
        b.append(int(i))
      elif tmp==2:
        c.append(int(i))
      elif tmp==3:
        d.append(int(i))

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

    # for i in range(int(a),int(b)):
    #   if i==int(c):
    #     tmp2=True
    #   if tmp2 and i==int(d):
    #     counter=counter+1
    #     tmp2=False
    #     break
    # for i in range(int(c),int(d)):
    #   if i==int(a):
    #     tmp2=True
    #   if tmp2 and i==int(b):
    #     counter=counter+1
    #     tmp2=False
    #     break
    if a<=b and b<=a or b<=a and c<=d:
      counter += 1
    if not (c < b or a > d):
        counter2 += 1
    a,b,c,d=[],[],[],[]

    tmp=0
  print(counter)
#[
var haha = "haahaha"

echo "Hello", haha," World"
"Hello ".echo "World"
echo("Hello World")
"Hello World".echo
write(stdout, "Hello World\n")
/////////////////////////////////////////////////////////////////////
var my_Var = "Muhaha"

echo my_Var
echo myvar
echo myVar
echo mYVAR
echo m_y_v_A_r
/////////////////////////////////////////////////////////////////////
var
  hello = "hello"
  world = "world"
  sp = " "
  full = hello & sp & world

full.add(hello)
echo full[6..full.high]
/////////////////////////////////////////////////////////////////////
import strformat

let
  name = "Ilian"
  age = 17
  food = "Lasange"

echo &"My name is {name} and\nI am {age} years old and like {food}"
/////////////////////////////////////////////////////////////////////
let x:int = 0

case x
of 1, 2, 3, 4:
  echo "Is 1,2,3,4"
of 5:
  echo "Is 5"
else:
  echo "Just another Number"
/////////////////////////////////////////////////////////////////////
import times, asyncdispatch

var tomorrow = now() + 1.days
tomorrow = now() + 1.days - 12.hours
echo tomorrow
]#

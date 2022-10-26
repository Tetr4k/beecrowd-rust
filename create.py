#!/usr/bin/env python3
import sys
from os import system

program = sys.argv[1]
cases = int(sys.argv[2])

system("mkdir "+program)
system("cp main.rs "+program)

for x in range(cases):
	system("touch ./"+program+"/case"+str(x+1)+".txt")
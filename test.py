#!/usr/bin/env python3
import sys
from os import system, listdir

program = sys.argv[1]
files = listdir(program)

command = "rustc ./"+str(program)+"/main.rs"

system(command)

if files:
	for f in files:
		if ".txt" in f:
			system("./main < ./"+program+"/"+f)
else:
	system("./main")
system("rm ./main")
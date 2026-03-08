#!/bin/python3
import os
import subprocess

# Calculate all categories
tables = {}
returncode = 0
for category in ['argparser', 'serializer', 'logging', 'regex', 'http-client', 'http-server']:
    print(f'\n{category}:')

    os.chdir(category)
    
    for name in sorted(os.listdir('.')):
        if os.path.isdir(name):
            print(f"  {name}: ", end='', flush=True)

            os.chdir(name)

            subprocess.run(["cargo", "update", "-q"])
            result = subprocess.run(["cargo", "outdated", "--exit-code=1"])

            if result.returncode > 0:
                returncode = 1                

            os.chdir('..')

    os.chdir('..')

exit(returncode)

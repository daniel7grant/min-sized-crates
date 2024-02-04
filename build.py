#!/bin/python3
import json
import os
import re
import subprocess
import time

FORCE_NO_CACHE = os.environ.get('FORCE_NO_CACHE', "false") != "false"
SIZE_FILENAME = '.size.json'

def calculate_size(name):
    os.chdir(name)

    if not FORCE_NO_CACHE and os.path.exists(SIZE_FILENAME):
        with open(SIZE_FILENAME, 'r') as file:
            size = json.load(file)
            os.chdir('..')
            return size

    if FORCE_NO_CACHE:
        subprocess.run(["cargo", "clean", "-q"])

    start_time = time.time()
    subprocess.run(["cargo", "build", "--release", "-q"])
    compile_time = time.time() - start_time
    
    compile_size = os.path.getsize(f"target/release/{name}")
    
    tree = subprocess.run(["cargo", "tree", "--prefix=none", "--no-dedupe"], capture_output=True)
    dependencies = len(set(tree.stdout.splitlines())) - 1
    
    size = { 'name': name, 'compile_size': compile_size, 'compile_time': compile_time, 'dependencies': dependencies }
    
    with open(SIZE_FILENAME, 'w') as file:
        json.dump(size, file)

    os.chdir('..')

    return size


def generate_markdown_table(sizes, baseline):
    rows = ""
    for size in sizes.values():
        size_increase = int((size['compile_size'] - baseline['compile_size']) / 1024)
        time_increase = size['compile_time'] - baseline['compile_time']
        rows += f"{size['name']} | +{size_increase}kB | +{time_increase:.2f}s | {size['dependencies']}\n"

    return f"""Name | Size | Compile time | Dependency count
---|:-:|:-:|:-:
{rows}"""


sizes = { 'baseline': calculate_size('baseline') }
tables = {}
for category in ['argparser']:
    os.chdir(category)
    
    sizes[category] = {}
    for name in sorted(os.listdir('.')):
        if os.path.isdir(name):
            sizes[category][name] = calculate_size(name)

    os.chdir('..')

    tables[category] = generate_markdown_table(sizes[category], sizes['baseline'])


start = re.compile(r'<details id="([a-z]*)">')
end = re.compile(r'</details>')
with open('README.md', 'r') as infile:
    with open('README.results.md', 'w') as outfile:
        current_id = None
        for line in infile:
            if current_id is None:
                outfile.write(line)

                match = start.match(line)
                if match:
                    current_id = match.group(1)
                    outfile.write(f"""<summary>Detailed comparison between crates</summary>

{tables[current_id]}
</details>
""")
                    
            else:
                if end.match(line):
                    current_id = None



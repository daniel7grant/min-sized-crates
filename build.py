#!/bin/python3
import json
import os
import platform
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

def generate_span_information(baseline):
    information = {
        'baseline-size': f"{int(baseline['compile_size'] / 1024)}kB",
        'baseline-example-size': f"+{500 - int(baseline['compile_size'] / 1024)}kB",
        'arch': platform.machine(),
        'os': platform.platform(),
        'rust-version': subprocess.run(["rustc", "--version"], capture_output=True).stdout.decode("utf-8").strip(),
    }

    def generate_span_information_inner(matches):
        name = matches.group(1)
        if name in information:
            return f'<span id="information/{name}">{information[name]}</span>'
        else:
            return matches.group(0)

    return generate_span_information_inner

def generate_span_sizes(sizes, baseline):
    def generate_span_sizes_inner(matches):
        category = matches.group(1)
        name = matches.group(2)

        if category in sizes and name in sizes[category]:
            size = sizes[category][name]
            size_increase = int((size['compile_size'] - baseline['compile_size']) / 1024)

            return f'<span id="{category}/{name}">+{size_increase}kB</span>'
        else:
            return matches.group(0)


    return generate_span_sizes_inner


def generate_span_percents(sizes, baseline):
    def generate_span_percents_inner(matches):
        category = matches.group(1)
        name = matches.group(2)
        basename = matches.group(3)

        if category in sizes and name in sizes[category] and basename in sizes[category]:
            size = sizes[category][name]['compile_size'] - baseline['compile_size']
            basesize = sizes[category][basename]['compile_size'] - baseline['compile_size']
            percent = float(size - basesize) / basesize * 100
            return f'<span id="{category}/{name}/{basename}">{percent:.2f}%</span>'
        else:
            return matches.group(0)

    return generate_span_percents_inner


# Calculate baseline
print("baseline: ", end='', flush=True)
baseline = calculate_size('baseline')
sizes = { 'baseline': baseline }
print(f"{int(baseline['compile_size'] / 1024)}kB {baseline['compile_time']:.2f}s")

# Calculate all categories
tables = {}
for category in ['argparser', 'serializer', 'logging', 'http-client', 'http-server']:
    print(f'\n{category}:')

    os.chdir(category)
    
    sizes[category] = {}
    for name in sorted(os.listdir('.')):
        if os.path.isdir(name):
            print(f"  {name}: ", end='', flush=True)
            size = calculate_size(name)
            sizes[category][name] = size

            size_increase = int((size['compile_size'] - baseline['compile_size']) / 1024)
            time_increase = size['compile_time'] - baseline['compile_time']
            print(f"+{size_increase}kB +{time_increase:.2f}s ({size['dependencies']} dependencies)")

    os.chdir('..')

    tables[category] = generate_markdown_table(sizes[category], sizes['baseline'])


# Replace tables with documentation
start = re.compile(r'<details id="([a-z_-]*)">')
end = re.compile(r'</details>')
# Replace spans with sizes
information = re.compile(r'<span id="information/([a-z_-]*)">\w*</span>')
size = re.compile(r'<span id="([a-z_-]*)/([a-z_-]*)">\w*</span>')
percent = re.compile(r'<span id="([a-z_-]*)/([a-z_-]*)/([a-z_-]*)">\w*</span>')
replace_span_information = generate_span_information(baseline)
replace_span_sizes = generate_span_sizes(sizes, baseline)
replace_span_percents = generate_span_percents(sizes, baseline)
with open('README.md', 'r') as infile:
    with open('README.results.md', 'w') as outfile:
        current_id = None
        for line in infile:
            # Replace <span id="name">information</span> with real information about the baseline or platform
            line = information.sub(replace_span_information, line)

            # Replace <span id="category/name">size</span> with real sizes in kb
            line = size.sub(replace_span_sizes, line)

            # Replace <span id="category/name/base">-percent%</span> with real percents
            line = percent.sub(replace_span_percents, line)

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



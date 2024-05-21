# convert a csv file to tsv

import sys

def convert(input_file, output_file):
    with open(input_file, 'r') as f:
        lines = f.readlines()
    with open(output_file, 'w') as f:
        for line in lines:
            f.write(line.replace(',', '\t'))

if __name__ == '__main__':
    convert(sys.argv[1], sys.argv[2])

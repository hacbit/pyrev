import ast
import os
import sys
from subprocess import Popen, PIPE

def cmp_rev_result(example: str):
    txt = example + '.txt'
    py = example + '.py'

    assert os.path.exists(txt) and os.path.exists(py), "This example not found"

    proc = Popen(['cargo', 'run', '--', '--file', txt], stdin=PIPE, stdout=PIPE, stderr=PIPE)

    res = proc.stdout.read()

    res_ast = ast.parse(res)

    with open(py, 'r', encoding='utf-8') as reader:
        content = reader.read()

    origin_ast = ast.parse(content)

    if ast.dump(origin_ast) == ast.dump(res_ast):
        print("[+] ok")
    else:
        print("[-] error")
        print(f"Expected: {content}")
        print(f"Get: {res}")

        print('Target AST:')
        print(origin_ast)

        print('Now AST:')
        print(res_ast)

def main():
    assert os.path.exists('test/')

    example = 'test/' + sys.argv[1]

    cmp_rev_result(example)

if __name__ == "__main__":
    main()

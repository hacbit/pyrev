from pwn import *
context(os='linux', arch='amd64', log_level='debug')

shellcode = '''
    mov rax, 0x68732f6e69622f
    push rax
    mov rdi, rsp
    mov rax, 0x1b
    syscall
'''
print(asm(shellcode))

import requests

url = 'https://eval.example.com'
data = {
    'user': 'admin',
    'password': 'admin123',
    'type': '1'
}
s = requests.Session()
r = s.post(url, data=data)
print(r.content.decode('utf-8'))

# coding:utf-8

import base64
import re
from Crypto.Cipher import AES
key = 'ABCDEFGHIJKLMNOP'


def AES_Decrypt(data):
    vi = '0102030405060708'  # 16位
    data = data.encode('utf8')
    print(data)
    encodebytes = base64.urlsafe_b64decode(data)
    print(encodebytes)
    # 将加密数据转换为bytes类型数据
    cipher = AES.new(key.encode('utf8'), AES.MODE_CBC, vi.encode('utf8'))
    text_decrypted = cipher.decrypt(encodebytes)
    print('text_decrypted:', text_decrypted)
    # 判断是否含有中文
    zhmodel = re.compile(u'[\u4e00-\u9fff]')
    match = zhmodel.search(text_decrypted)
    if match == False:
        # 无中文时补位
        unpad = lambda s: s[0:-s[-1]]
        text_decrypted = unpad(text_decrypted)
    text_decrypted = text_decrypted.decode('utf8').rstrip()  # 去掉补位的右侧空格
    return text_decrypted

print(AES_Decrypt("F8dIExttsNN1WK5BCLHhkRZVwVIoI70W1G+pjPhgNRk="))
# 结果：需加密的字符串


def AES_Encrypt(data):
    vi = '0102030405060708'  # 16位
    cryptor = AES.new(key.encode('utf8'), AES.MODE_CBC, vi.encode('utf8'))
    # 判断是否含有中文
    zhmodel = re.compile(u'[\u4e00-\u9fff]')
    match = zhmodel.search(data)
    if match == None:
        # 无中文时
        add = 16 - len(data) % 16
        pad = lambda s: s + add * chr(add)
        data = pad(data)
        enctext = cryptor.encrypt(data.encode('utf8'))
    else:
        # 含有中文时
        data = data.encode()
        add = 16 - len(data) % 16
        data = data + add * (chr(add)).encode()
        enctext = cryptor.encrypt(data)
    encodestrs = base64.b64encode(enctext).decode('utf8')
    return encodestrs

print(AES_Encrypt('abcd123'))
# 结果：KeAJB/1sMcGk6HUmKeC5jA==
# python解密可直接调用print(AES_Decrypt("KeAJB/1sMcGk6HUmKeC5jA=="))

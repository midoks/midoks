# coding:utf-8

import os

from cryptography.hazmat.primitives import padding
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.backends import default_backend


class AESCrypto(object):

    AES_CBC_KEY = os.urandom(32)
    AES_CBC_IV = os.urandom(16)

    @classmethod
    def encrypt(cls, data, mode='cbc'):
        func_name = '{}_encrypt'.format(mode)
        func = getattr(cls, func_name)
        return func(data)

    @classmethod
    def decrypt(cls, data, mode='cbc'):
        func_name = '{}_decrypt'.format(mode)
        func = getattr(cls, func_name)
        return func(data)

    @staticmethod
    def pkcs7_padding(data):
        if not isinstance(data, bytes):
            data = data.encode()

        padder = padding.PKCS7(algorithms.AES.block_size).padder()

        padded_data = padder.update(data) + padder.finalize()

        return padded_data

    @classmethod
    def cbc_encrypt(cls, data):
        if not isinstance(data, bytes):
            data = data.encode()

        cipher = Cipher(algorithms.AES(cls.AES_CBC_KEY),
                        modes.CBC(cls.AES_CBC_IV),
                        backend=default_backend())
        encryptor = cipher.encryptor()

        padded_data = encryptor.update(cls.pkcs7_padding(data))

        return padded_data

    @classmethod
    def cbc_decrypt(cls, data):
        if not isinstance(data, bytes):
            data = data.encode()

        cipher = Cipher(algorithms.AES(cls.AES_CBC_KEY),
                        modes.CBC(cls.AES_CBC_IV),
                        backend=default_backend())
        decryptor = cipher.decryptor()

        uppaded_data = cls.pkcs7_unpadding(decryptor.update(data))

        uppaded_data = uppaded_data.decode()
        return uppaded_data

    @staticmethod
    def pkcs7_unpadding(padded_data):
        unpadder = padding.PKCS7(algorithms.AES.block_size).unpadder()
        data = unpadder.update(padded_data)

        try:
            uppadded_data = data + unpadder.finalize()
        except ValueError:
            raise Exception('无效的加密信息!')
        else:
            return uppadded_data


if __name__ == "__main__":
    r = AESCrypto.encrypt("abcd123")
    print(r)

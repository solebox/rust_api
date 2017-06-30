#!/usr/bin/env python
import base64
import sys
import crypto 
import sys
sys.modules['Crypto'] = crypto
import hashlib
from Crypto import Random
from Crypto.Cipher import AES


class AESCipher(object):

    def __init__(self, key): 
        self.bs = 16 
        self.key = hashlib.sha256(key.encode()).digest()
        res = [ord(k) for k in self.key]
        print(res)
    def encrypt(self, raw):
        raw = self._pad(raw)
        iv = Random.new().read(AES.block_size)
        cipher = AES.new(self.key, AES.MODE_CBC, iv)
        return base64.b64encode(cipher.encrypt(raw) + iv)

    def decrypt(self, enc):
        enc = base64.b64decode(enc)
        iv = enc[len(enc)-AES.block_size:]
        cipher = AES.new(self.key, AES.MODE_CBC, iv)
        return self._unpad(cipher.decrypt(enc[:len(enc)-AES.block_size])).decode('utf-8')

    def _pad(self, s):
        return s + (self.bs - len(s) % self.bs) * chr(self.bs - len(s) % self.bs)

    @staticmethod
    def _unpad(s):
        return s[:-ord(s[len(s)-1:])]

if __name__ == "__main__":
    aes = AESCipher("klutsh")
    encrypted = aes.encrypt('{"service":"ballz sackz"}')
    decrypted = aes.decrypt(encrypted)
    print(decrypted)

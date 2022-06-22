# Node chaha20 192bit
This library will allow you use a XchaCha20Poly1305 with 192bit (24 bytes) encryption

# Quickstart
install with
```
npm i --save node-chacha20-192
```
and use this
```
const { encrypt, decrypt, getXchaCha20Poly1305Cipher } = require('node-chacha20-192')

// generate with: openssl rand 32 | base64 -w 0
const key = '14wJ5sfw+TXBHmmWk4RU9AUixM46TWxr1wqRvcenCdc='
// generate with: openssl rand 24 | base64 -w 0
const nonce = 'y8moUsO1lLYTHaCnPFh7KA7HCvAnMaTE'
const data = 'my secret text'

const keyBuffer = Buffer.from(key, 'base64')
const nonceBuffer = Buffer.from(nonce, 'base64')
const dataBuffer = Buffer.from(data, 'utf8')

const cipher = getXchaCha20Poly1305Cipher(keyBuffer)
const encr = encrypt(cipher, nonceBuffer, dataBuffer)
console.log(`encrypted text (base64): ${encr.toString('base64')}`)
const decr = decrypt(cipher, nonceBuffer, encr)
console.log(`decrypted text: ${Buffer.from(decr).toString()}`)
```

# Why?
The other node.js implementations I found where using only up to 96bit (12 bytes) encryption

# How?
This library relies on rust chacha20poly1305 to provide 192bit encryption

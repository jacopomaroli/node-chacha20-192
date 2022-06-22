import test from 'ava'

import { encrypt, decrypt, getXchaCha20Poly1305Cipher } from '../index.js'

test('XchaCha20 196bit', (t) => {
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
  // console.log(`encrypted text (base64): ${encr.toString('base64')}`)
  const decr = decrypt(cipher, nonceBuffer, encr)
  const decrStr = Buffer.from(decr).toString()
  // console.log(`decrypted text: ${decrStr}`)
  t.is(data, decrStr)
})

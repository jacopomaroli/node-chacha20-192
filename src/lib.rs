#![deny(clippy::all)]
#![allow(dead_code)]

#[macro_use]
extern crate napi_derive;

use chacha20poly1305::{
  aead::{Aead, NewAead},
  Key, XChaCha20Poly1305, XNonce,
};

use napi::{Env, JsBuffer, JsObject, Result};

#[napi]
fn get_xcha_cha20_poly1305_cipher(env: Env, key: JsBuffer) -> Result<JsObject> {
  let mut js_obj = env.create_object()?;
  let ctx = XChaCha20Poly1305::new(&Key::clone_from_slice(&key.into_value().unwrap()));
  env.wrap(&mut js_obj, ctx)?;
  Ok(js_obj)
}

#[napi]
fn encrypt(
  env: Env,
  js_cipher: JsObject,
  nonce: JsBuffer,
  plaintext: JsBuffer,
) -> Result<JsBuffer> {
  let cipher: &mut XChaCha20Poly1305 = env.unwrap(&js_cipher)?;
  let res = cipher
    .encrypt(
      &XNonce::clone_from_slice(&nonce.into_value().unwrap()),
      plaintext.into_value().unwrap().as_ref(),
    )
    .expect("encryption failure");
  let buffer = env.create_buffer_with_data(res)?.into_raw();
  Ok(buffer)
}

#[napi]
fn decrypt(
  env: Env,
  js_cipher: JsObject,
  nonce: JsBuffer,
  ciphertext: JsBuffer,
) -> Result<JsBuffer> {
  let cipher: &mut XChaCha20Poly1305 = env.unwrap(&js_cipher)?;
  let res = cipher
    .decrypt(
      &XNonce::clone_from_slice(&nonce.into_value().unwrap()),
      ciphertext.into_value().unwrap().as_ref(),
    )
    .expect("decryption failure");
  let buffer = env.create_buffer_with_data(res)?.into_raw();
  Ok(buffer)
}

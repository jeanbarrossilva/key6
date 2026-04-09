// Copyright © Jean Silva
//
// This file is part of the key6 open-source project.
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see https://www.gnu.org/licenses.
//

use crate::key::Key;
use argon2::Argon2;
use argon2::PasswordHasher;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

pub struct Keychain<'a> {
  hasher: Argon2<'a>,
  hashed_seed_salt: SaltString,
  storage: Vec<Key>
}

impl<'a> Keychain<'a> {
  const SEED_MIN_LENGTH: usize = 8;

  fn new(seed: String) -> Result<Self, ShortSeedError> {
    let seed_length = seed.len();
    if seed_length < Self::SEED_MIN_LENGTH {
      return Err(ShortSeedError { seed_length })
    }
    let hasher = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    Ok(Self {
      hasher,
      hashed_seed_salt: salt,
      storage: vec![]
    })
  }

  fn is_empty(&self) -> bool {
    self.storage.is_empty()
  }

  fn contains(&self, key: Key) -> bool {
    self.storage.contains(&key)
  }

  fn store(&mut self, name: String, login: String, password: String) -> &Key {
    let encrypted_password = self
      .hasher
      .hash_password(password.as_bytes(), &self.hashed_seed_salt)
      .unwrap()
      .to_string();
    let key = Key::generate(name, login, encrypted_password);
    self.storage.push(key);
    self.storage.get(self.storage.len() - 1).unwrap()
  }

  fn remove(&mut self, key: Key) {
    for index in 0..self.storage.len() {
      let candidate_key = self.storage.get(index).unwrap();
      if *candidate_key != key {
        continue;
      }
      self.storage.remove(index);
    }
  }
}

#[derive(PartialEq, Debug)]
struct ShortSeedError {
  seed_length: usize
}

impl Display for ShortSeedError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Seed for keychain should contain at least {} characters; given one had \
       only {}.",
      Keychain::SEED_MIN_LENGTH,
      self.seed_length
    )
  }
}

impl Error for ShortSeedError {}

#[cfg(test)]
mod tests {
  use crate::key::Key;
  use crate::keychain::Keychain;
  use crate::keychain::ShortSeedError;

  const DUMMY_KEY_DECRYPTED_PASSWORD: &str = "123";

  #[test]
  fn errors_if_seed_is_less_than_eight_characters_long() {
    assert_eq!(
      Keychain::new(String::from("apollo")).err(),
      Some(ShortSeedError { seed_length: 6 })
    );
  }

  #[test]
  fn is_empty_by_default() {
    let keychain = create_keychain();
    assert!(keychain.is_empty())
  }

  #[test]
  fn stores() {
    let mut keychain = create_keychain();
    let key = store_and_clone_dummy_key(&mut keychain);
    assert!(keychain.contains(key))
  }

  #[test]
  fn is_not_empty_after_storing_first_key() {
    let mut keychain = create_keychain();
    let _ = store_and_clone_dummy_key(&mut keychain);
    assert!(!keychain.is_empty())
  }

  #[test]
  fn generates_random_id_for_each_stored_key() {
    let mut keychain = create_keychain();
    for index in 0..4 {
      let key = store_and_clone_dummy_key(&mut keychain);
      if index == 0 {
        continue
      }
      assert_ne!(key.id, keychain.storage[index - 1].id)
    }
  }

  #[test]
  fn password_of_stored_key_is_hashed() {
    let mut keychain = create_keychain();
    let key = store_and_clone_dummy_key(&mut keychain);
    assert_ne!(key.hashed_password, DUMMY_KEY_DECRYPTED_PASSWORD)
  }

  #[test]
  fn removes() {
    let mut keychain = create_keychain();
    let key = store_and_clone_dummy_key(&mut keychain);
    keychain.remove(key.clone());
    assert!(!keychain.contains(key))
  }

  #[test]
  fn is_empty_after_removing_last_key() {
    let mut keychain = create_keychain();
    let key = store_and_clone_dummy_key(&mut keychain);
    keychain.remove(key);
    assert!(keychain.is_empty())
  }

  fn create_keychain<'a>() -> Keychain<'a> {
    Keychain::new(String::from("artemis-ii")).unwrap()
  }

  fn store_and_clone_dummy_key(keychain: &mut Keychain) -> Key {
    (*keychain.store(
      String::from("NASA"),
      String::from("key6"),
      String::from(DUMMY_KEY_DECRYPTED_PASSWORD)
    ))
    .clone()
  }
}

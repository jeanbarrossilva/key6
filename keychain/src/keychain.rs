//
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

pub struct Keychain {
  storage: Vec<Key>
}

impl Keychain {
  fn new() -> Self {
    Self { storage: vec![] }
  }

  fn is_empty(&self) -> bool {
    self.storage.is_empty()
  }

  fn contains(&self, key: Key) -> bool {
    self.storage.contains(&key)
  }

  fn store(&mut self, name: String, login: String, password: String) -> &Key {
    let key = Key::generate(name, login, password);
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

#[cfg(test)]
mod tests {
  use crate::key::Key;
  use crate::keychain::Keychain;

  #[test]
  fn is_empty_by_default() {
    let keychain = Keychain::new();
    assert!(keychain.is_empty())
  }

  #[test]
  fn stores() {
    let mut keychain = Keychain::new();
    let key = store_and_clone_dummy_key(&mut keychain);
    assert!(keychain.contains(key))
  }

  #[test]
  fn generates_random_id_for_each_stored_key() {
    let mut keychain = Keychain::new();
    for index in 0..=128 {
      let key = store_and_clone_dummy_key(&mut keychain);
      if index == 0 {
        continue
      }
      assert_ne!(key.id, keychain.storage[index - 1].id)
    }
  }

  #[test]
  fn is_not_empty_after_storing_first_key() {
    let mut keychain = Keychain::new();
    let _ = store_and_clone_dummy_key(&mut keychain);
    assert!(!keychain.is_empty())
  }

  #[test]
  fn removes() {
    let mut keychain = Keychain::new();
    let key = store_and_clone_dummy_key(&mut keychain);
    keychain.remove(key.clone());
    assert!(!keychain.contains(key))
  }

  #[test]
  fn is_empty_after_removing_last_key() {
    let mut keychain = Keychain::new();
    let key = store_and_clone_dummy_key(&mut keychain);
    keychain.remove(key);
    assert!(keychain.is_empty())
  }

  fn store_and_clone_dummy_key(keychain: &mut Keychain) -> Key {
    (*keychain.store(
      String::from("key6"),
      String::from("jeanbarrossilva"),
      String::from("123")
    ))
    .clone()
  }
}

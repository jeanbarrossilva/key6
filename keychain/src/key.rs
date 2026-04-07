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

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct Key {
  pub id: String,
  pub name: String,
  pub login: String,
  pub password: String
}

impl Key {
  pub fn new(
    id: String,
    name: String,
    login: String,
    password: String
  ) -> Result<Key, InvalidIdentifierFormatError> {
    let uuid = Uuid::try_parse(id.as_str());
    if uuid.is_err() {
      Err(InvalidIdentifierFormatError::new(id))
    } else {
      Ok(Key::new_without_validation(
        uuid.unwrap().to_string(),
        name,
        login,
        password
      ))
    }
  }

  pub fn generate(name: String, login: String, password: String) -> Self {
    Key::new_without_validation(
      Uuid::new_v4().to_string(),
      name,
      login,
      password
    )
  }

  fn new_without_validation(
    id: String,
    name: String,
    login: String,
    password: String
  ) -> Self {
    Key {
      id,
      name,
      login,
      password
    }
  }
}

#[derive(Debug)]
pub struct InvalidIdentifierFormatError {
  id: String
}

impl InvalidIdentifierFormatError {
  fn new(id: String) -> Self {
    Self { id }
  }
}

impl Display for InvalidIdentifierFormatError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.id)
  }
}

impl Error for InvalidIdentifierFormatError {}

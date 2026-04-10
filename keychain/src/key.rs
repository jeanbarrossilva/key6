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
  pub hashed_password: String
}

impl Key {
  pub fn new(
    id: String,
    name: String,
    login: String,
    hashed_password: String
  ) -> Result<Self, UuidV4FormatError> {
    let uuid = Uuid::try_parse(id.as_str());
    if uuid.is_err() {
      Err(UuidV4FormatError::new(id))
    } else {
      Ok(Key {
        id: uuid.unwrap().to_string(),
        name,
        login,
        hashed_password
      })
    }
  }

  pub fn generate(
    name: String,
    login: String,
    hashed_password: String
  ) -> Self {
    let id = Uuid::new_v4().to_string();
    Key {
      id,
      name,
      login,
      hashed_password
    }
  }
}

#[derive(Debug)]
pub struct UuidV4FormatError {
  id: String
}

impl UuidV4FormatError {
  fn new(id: String) -> Self {
    Self { id }
  }
}

impl Display for UuidV4FormatError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.id)
  }
}

impl Error for UuidV4FormatError {}

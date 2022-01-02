// 
//  Stigmee: A 3D browser and decentralized social network.
//  Copyright 2021-2022 Philippe Anel <zexigh@gmail.com>
// 
//  This file is part of Stigmee.
// 
//  Project : Stigmark
//  Version : 0.0-1
// 
//  Stigmee is free software: you can redistribute it and/or modify it
//  under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
// 
//  This program is distributed in the hope that it will be useful, but
//  WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
//  General Public License for more details.
// 
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <http://www.gnu.org/licenses/>.
// 

use jsonwebtoken::errors::Error;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize, // expiration time
    pub uid: u32,
}

#[allow(dead_code)]
impl Claims {
    pub fn decode_from(token: String) -> Result<Claims, String> {
        // todo: remove unwrap
        let claims = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default());
        if let Err(err) = claims {
            return Err(format!("could not decode token: {}", err));
        }
        Ok(claims.unwrap().claims)
    }
}

#[allow(dead_code)]
pub fn create_token(user_id: u32) -> Result<String, Error> {
    let my_claims = Claims {
        uid: user_id,
        exp: 10000000000, // TODO
    };
    encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))
}

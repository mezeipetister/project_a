// Copyright (C) 2019 Peter Mezei <github.com/mezeipetister>.
// All rights reserved. License: GNU GPLv2.0.

// Project A: an MVP project about publishing agri tech - mainly machines - information.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

package user

import (
	"fmt"
	"github.com/dgrijalva/jwt-go"
	"time"
)

// Create the JWT key used to create the signature
var jwtKey = []byte("my_secret_key")

// Credentials create a struct to read
// the username and password from the request body
type credentials struct {
	Password string `json:"password"`
	Username string `json:"username"`
}

// Claims create a struct that will be
// encoded to a JWT. We add jwt.StandardClaims
// as an embedded type, to provide fields like expiry time
type claims struct {
	Username string `json:"username"`
	jwt.StandardClaims
}

// CreateToken create a token based on
// a given username
func CreateToken(username string) (string, error) {
	// Declare the expiration time of the token
	// here, we have kept it as 5 minutes
	expirationTime := time.Now().Add(5 * time.Minute)

	// Create the JWT claims, which includes the username and expiry time
	claims := &claims{
		Username: username,
		StandardClaims: jwt.StandardClaims{
			// In JWT, the expiry time is expressed as unix milliseconds
			ExpiresAt: expirationTime.Unix(),
		},
	}

	// Declare the token with the algorithm used for signing, and the claims
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	// Create the JWT string
	tokenString, err := token.SignedString(jwtKey)
	if err != nil {
		// If there is an error in creating the JWT return an internal server error
		return "", fmt.Errorf("Error during JWT token creation. Error msg: %s", err.Error())
	}

	return tokenString, nil
}

// ValidateToken gets token as string and a claims pointer
// validate token, if error occures, then returns it,
// if token is valid, puts its claims content into the
// claims pointer
func ValidateToken(tokenString string, claims *claims) error {
	tkn, err := jwt.ParseWithClaims(tokenString, claims, func(token *jwt.Token) (interface{}, error) {
		return jwtKey, nil
	})
	if err != nil {
		return fmt.Errorf("Error occured while validate Token, error message is: %s",
			jwt.ErrSignatureInvalid)
	}
	if !tkn.Valid {
		return fmt.Errorf("Error while validate JWT token. Token is invalid")
	}
	return nil
}

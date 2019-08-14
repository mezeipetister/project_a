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

import "testing"

// Test JWT token creation based on username
// Test: creation and token length
func TestGetToken(t *testing.T) {
	username := "loremIpsum"
	token, err := CreateToken(username)
	if err != nil {
		t.Errorf("Error while creating JWT token. Message is: %s", err)
	}
	if len(token) == 0 {
		t.Errorf("Token error, len is 0")
	}
}
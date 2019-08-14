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

package main

import (
	"fmt"
	"github.com/mezeipetister/project_a/user"
	"html/template"
	"net/http"
	"time"
)

// templates
var templates = template.Must(template.ParseGlob("templates/*"))

func index(w http.ResponseWriter, r *http.Request) {
	type _data struct {
		Title string
	}
	data := _data{Title: "LoremIpsumDolorem"}
	templates.ExecuteTemplate(w, "base", data)
}

// Counter is a test struct
type Counter struct {
	i int
	a int
}

func waitCounter(done chan bool) {
	// Counter variable counter
	counter := Counter{0, 0}
	for i := 0; i < 10; i++ {
		counter.i++
		counter.a += 3
		time.Sleep(1 * time.Second)
	}
	done <- true
}

func sumTwo(a, b int) int {
	return a + b
}

func modAandB(a, b *int) {
	*a = 8
	*b = 9
}

// lorem asd
// TODO: Wohooo
func main() {

	a, b := 1, 2

	c, d := &a, &b

	modAandB(c, d)

	fmt.Printf("A: %d, B: %d\n",
		a, b)

	fmt.Printf("Sum of 2, 7 is: %d",
		sumTwo(2, 7))

	token, _ := user.CreateToken("LoremIpsum")
	fmt.Printf("Token demo is: %s",
		token)

	done := make(chan bool)
	go waitCounter(done)
	<-done

	// 	fmt.Println("Project A\nLicense:\tGNU GPLv2.0\nAuthor:\t\tPeter Mezei")
	// 	http.HandleFunc("/", index)
	// 	http.ListenAndServe(":8080", nil)
}

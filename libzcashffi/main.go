package main

import (
	// #cgo LDFLAGS: -L./include -lzcashffi
	// #include "include/zcashffi.h"
	"C"
	_ "embed"
)
import "unsafe"

//go:embed params/sapling-output.params
var saplingOutput []byte

//go:embed params/sapling-spend.params
var saplingSpend []byte

func main() {
	/*
		person := &C.struct_Person{
			name: C.CString("hello person"),
		}
		C.hello(person)

		log.Println("read from rust", C.GoString(C.get_person_name(person)))
		C.set_person_name(person, C.CString("hello go"))
		log.Println(C.GoString(person.name))

		people := []*C.struct_Person{
			&C.struct_Person{
				name: C.CString("hello person 1"),
			},
			&C.struct_Person{
				name: C.CString("hello person 2"),
			},
			&C.struct_Person{
				name: C.CString("hello person 3"),
			},
		}
		C.hello_vec(C.uint32_t(len(people)), people[0])
	*/
	C.sapling((*C.uint8_t)(unsafe.Pointer(&saplingOutput[0])), C.uint32_t(len(saplingOutput)))
}

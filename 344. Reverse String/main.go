package main

import (
        "fmt"
)

func reverseString(s string) string {
        ret := ""
        for i := len(s)-1; i >= 0; i-- {
                ret += string(s[i])
        }
        return ret
}

func main (){
        expect := "olleh"
        actual := reverseString("hello")
        if expect != actual {
                fmt.Printf("expect:%v, actual:%v\n", expect, actual)
        }
}

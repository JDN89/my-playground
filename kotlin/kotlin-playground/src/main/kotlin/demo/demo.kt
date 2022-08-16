package demo

import java.awt.Container

data class Person(val id:Int, var email:String)


fun main() {                        // 2
    println("Hello, World!")        // 3
    val person =  Person(1,"jan")
    println(person)
}

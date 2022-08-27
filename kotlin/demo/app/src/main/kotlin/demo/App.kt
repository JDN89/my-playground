/*
 * This Kotlin source file was generated by the Gradle 'init' task.
 */
package demo

import java.lang.IllegalArgumentException
import java.lang.NullPointerException
import java.util.IllegalFormatCodePointException

fun Person.printer (x: Person) {
     println(x)
}
data class Person(
    val firstName: String = "Jan",
    val lastName: String = "De Niels"
)
{

    fun printName () {
        println("${
            this.firstName
        }, ${
            this.lastName
        }")
    }
}

class Address() {
    var name: String = "Jan"
    var street: String = "Helen"
    var city: String = "London"
    var state: String? = null
    var zip: String = "123456"
}

fun copyAddress(address: Address): Address {
    val result = Address() // there's no 'new' keyword in Kotlin
    result.name = address.name // accessors are called
    result.street = address.street
    // ...
    return result
}


fun main() {
    val jan = Person()
    jan.printer(jan)

//    jan.printName()
}


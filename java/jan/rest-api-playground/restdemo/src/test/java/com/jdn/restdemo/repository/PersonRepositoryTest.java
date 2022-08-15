package com.jdn.restdemo.repository;

import com.jdn.restdemo.model.Address;
import com.jdn.restdemo.model.Person;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.orm.jpa.DataJpaTest;

import java.util.Optional;

import static org.assertj.core.api.Assertions.assertThat;

//to test against h2 db -> @DataJpaTest
@DataJpaTest
class PersonRepositoryTest {

    @Autowired
    private PersonRepository underTest;

    @Test
    void itShouldSelectPersonByEmail() {
        //Given
        //When
        //Then
    }

    @Test
    void itShouldSavePerson() {
        //Given

        Integer personId = (int) (Math.random() * 2) + 1;
        Integer addressId = (int) (Math.random() * 50 + 1);
        Address address = new Address(addressId, "street", "City");
        Person person = new Person(personId, "Jan", "De Niels", "jdn@gmail.com", address);

        //When
        Person save = underTest.save(person);
        System.out.println(save);

        //Then
        Optional<Person> optionalPerson = underTest.findPersonById(personId);
        assertThat(optionalPerson).isPresent();

    }
}
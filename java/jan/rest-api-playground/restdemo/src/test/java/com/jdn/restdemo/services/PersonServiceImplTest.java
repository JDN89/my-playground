package com.jdn.restdemo.services;

import com.jdn.restdemo.model.Address;
import com.jdn.restdemo.model.Person;
import com.jdn.restdemo.repository.PersonRepository;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.ArgumentCaptor;
import org.mockito.Captor;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;

import java.util.Optional;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.BDDMockito.given;
import static org.mockito.BDDMockito.then;
import static org.mockito.Mockito.never;

class PersonServiceImplTest {

    @Mock
    private PersonRepository personRepository;
    private AutoCloseable closeable;

    // Check if the mock receives the correct argument
    @Captor
    private ArgumentCaptor<Person> personArgumentCaptor;

    private PersonService underTest;

    @BeforeEach
    void setUp() {
        // alternative way to mock the repo
        closeable = MockitoAnnotations.openMocks(this);
        underTest = new PersonServiceImpl(personRepository);
    }

    @AfterEach
    void closeService() throws Exception {
        closeable.close();
    }

    @Test
    void itShouldSaveNewPerson() {
        //Given
        Integer personId = (int) (Math.random() * 2) + 1;
        Integer addressId = (int) (Math.random() * 50 + 1);
        String email = "jan.de.niels@ballekes";
        Address address = new Address(addressId, "street", "City");
        Person person = new Person(personId, "Jan", "De Niels", email, address);

        // No person found with the same email address
        given(personRepository.findPersonByEmail(email)).willReturn(Optional.empty());

        //When
        underTest.addNewPerson(person);
        //Then
        then(personRepository).should().save(personArgumentCaptor.capture());
        Person personArgumentCaptorValue = personArgumentCaptor.getValue();
        assertThat(personArgumentCaptorValue).isEqualTo(person);
    }

    @Test
    void itShouldNotSavePersonWhenPersonExists() {
        //Given
        String email = "jan.de.niels@ballekes";
        Address address = new Address("street", "City");
        Person person = new Person("Jan", "De Niels", email, address);

        given(personRepository.findPersonByEmail(email)).willReturn(Optional.of(person));

        //When

        //Then
        assertThatThrownBy(() -> underTest.addNewPerson(person))
                .isInstanceOf(IllegalStateException.class)
                .hasMessageContaining("Person already registered");

        then(personRepository).should(never()).save(any());

    }
}
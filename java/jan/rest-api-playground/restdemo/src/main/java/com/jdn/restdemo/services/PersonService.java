package com.jdn.restdemo.services;

import com.jdn.restdemo.exception.PersonNotFoundException;
import com.jdn.restdemo.model.Person;
import org.springframework.data.domain.Page;
import org.springframework.web.bind.annotation.RequestBody;

public interface PersonService {
    Iterable<Person> findAll();

    Page<Person> findAllOnPage(int page, int size);

    void addNewPerson(@RequestBody Person person);

    void deletePerson(Integer id);

    Person findPersonById(Integer id) throws PersonNotFoundException;

    Person updatePerson(int id, Person person);
}

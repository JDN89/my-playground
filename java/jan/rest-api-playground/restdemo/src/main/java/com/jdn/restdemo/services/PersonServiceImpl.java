package com.jdn.restdemo.services;

import com.jdn.restdemo.model.Person;
import com.jdn.restdemo.repository.PersonRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.PageRequest;
import org.springframework.stereotype.Service;

import java.util.Objects;
import java.util.Optional;

@RequiredArgsConstructor
@Service
public class PersonServiceImpl implements PersonService {

    private final PersonRepository repository;

    @Override
    public Iterable<Person> findAll() {
        //if you want to convert the iterable to a list
//        Iterable<Person> all = repository.findAll();
//        List<Person> list = Streamable.of(all).stream().toList();
        return repository.findAll();
    }

    @Override
    public Page<Person> findAllOnPage(int page, int size) {
        PageRequest pr = PageRequest.of(page, size);
        return repository.findAll(pr);
    }

    @Override
    public void addNewPerson(Person person) {

        Optional<Person> personByEmail = repository.findPersonByEmail(person.getEmail());

        if (personByEmail.isPresent()) {
            throw new IllegalStateException("Person already registered");
        }

        // check if not already present

        repository.save(person);
    }

    @Override
    public void deletePerson(Integer id) {
        Optional<Person> person = repository.findPersonById(id);
        if (person.isEmpty()) {
            throw new IllegalStateException("Person not found in db");
        }
        repository.deleteById(id);
    }

    @Override
    public Person findPersonById(Integer id) {
        Optional<Person> personById = repository.findPersonById(id);

        if (personById.isEmpty()) {
            throw new IllegalStateException("Person not found");
        }
        return personById.get();
    }

    // I update the Person and Address entitities at the same time and override all the fields
    // This can make an application slow if you have a lot of fields
    // solution 1:
    // I have a separate entity for address -> update address separately -> address service
    // Solution 2
    // Partial Data update (Map DTO to db) and only update the fields that have been changed
    // https://www.baeldung.com/spring-data-partial-update

    @Override
    public Person updatePerson(int id, Person person) {
        Person personDb = repository.findPersonById(id).get();
        if (Objects.nonNull(person.getFirstName())
                && !"".equalsIgnoreCase(person.getFirstName())) {
            personDb.setFirstName(person.getFirstName());
        }

        if (Objects.nonNull(person.getLastName())
                && !"".equalsIgnoreCase(person.getLastName())) {
            personDb.setLastName(person.getLastName());
        }

        if (Objects.nonNull(person.getEmail())
                && !"".equalsIgnoreCase(person.getEmail())) {
            personDb.setEmail(person.getEmail());
        }
        if (Objects.nonNull(person.getAddress().getAddress())
                && !"".equalsIgnoreCase(person.getAddress().getAddress())) {
            personDb.getAddress().setAddress(person.getAddress().getAddress());

        }
        if (Objects.nonNull(person.getAddress().getCity())
                && !"".equalsIgnoreCase(person.getAddress().getCity())) {
            personDb.getAddress().setCity(person.getAddress().getCity());
        }

        return repository.save(personDb);
    }

}

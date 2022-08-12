package com.jdn.restdemo.services;

import com.jdn.restdemo.model.Person;
import com.jdn.restdemo.repository.PersonRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.PageRequest;
import org.springframework.stereotype.Service;

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

}

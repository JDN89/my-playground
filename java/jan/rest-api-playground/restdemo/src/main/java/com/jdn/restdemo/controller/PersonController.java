package com.jdn.restdemo.controller;

import com.jdn.restdemo.exception.PersonNotFoundException;
import com.jdn.restdemo.model.Person;
import com.jdn.restdemo.services.PersonService;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Page;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/api/v1/people")
@RequiredArgsConstructor
public class PersonController {
    private final PersonService service;

    @GetMapping
    public Iterable<Person> findAll() {
        return service.findAll();
    }

    // http://localhost:8080/api/v1/people/page?page=0&size=4
    @GetMapping("/pagination")
    public Page<Person> findAllOnPage(@RequestParam int page, @RequestParam int size) {
        return service.findAllOnPage(page, size);
    }

    @GetMapping("/person/{id}")
    public Person findPersonById(@PathVariable("id") Integer id) throws PersonNotFoundException {
        return service.findPersonById(id);

    }

    @PostMapping()
    @ResponseStatus(HttpStatus.CREATED)
    public void addNewPerson(@RequestBody Person person) {
        service.addNewPerson(person);
    }

    @DeleteMapping("/person/{id}")
    public void deletePerson(@PathVariable("id") int id) {
        service.deletePerson(id);
    }

    @PutMapping("/person/{id}")
    public Person updatePerson(@PathVariable("id") int id, @RequestBody Person person) {
        return service.updatePerson(id, person);
    }
}

package com.jdn.restdemo.services;

import com.jdn.restdemo.model.Person;
import org.springframework.data.domain.Page;
import org.springframework.web.bind.annotation.RequestParam;

public interface PersonService {
    Iterable<Person> findAll();
    Page<Person> findAllOnPage(@RequestParam int page, @RequestParam int size, @RequestParam String sort);
}

package com.jdn.restdemo.controller;

import com.jdn.restdemo.model.Person;
import com.jdn.restdemo.services.PersonService;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Page;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

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
    @GetMapping("/page")
    public Page<Person> findAllOnPage(@RequestParam int page, @RequestParam int size) {
        return service.findAllOnPage(page,size);
    }
}

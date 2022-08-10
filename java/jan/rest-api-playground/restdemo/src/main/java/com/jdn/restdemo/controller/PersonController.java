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
@RequestMapping("/api/v1")
@RequiredArgsConstructor
public class PersonController {
    private final PersonService service;

    @GetMapping("/findall")
    public Iterable<Person> findAll() {
        return service.findAll();
    }

    @GetMapping
    public Page<Person> findAll(@RequestParam int page, @RequestParam int size, @RequestParam String sort) {
        return service.findAllOnPage(page,size,sort);
    }
}

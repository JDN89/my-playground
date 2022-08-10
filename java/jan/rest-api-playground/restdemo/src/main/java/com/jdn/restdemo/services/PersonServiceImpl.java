package com.jdn.restdemo.services;

import com.jdn.restdemo.model.Person;
import com.jdn.restdemo.repository.PersonRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;
import org.springframework.stereotype.Service;
import org.springframework.web.bind.annotation.RequestParam;

@RequiredArgsConstructor
@Service
public class PersonServiceImpl implements PersonService {

    private final PersonRepository repository;

    @Override
    public Iterable<Person> findAll() {
            return repository.findAll();
    }

    public Page<Person> findAllOnPage(@RequestParam int page, @RequestParam int size, @RequestParam String sort) {
        PageRequest pr = PageRequest.of(page,size, Sort.by(sort));
        return repository.findAll(pr);
    }

}

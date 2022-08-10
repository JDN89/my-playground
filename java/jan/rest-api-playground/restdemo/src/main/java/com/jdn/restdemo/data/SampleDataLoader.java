package com.jdn.restdemo.data;

import com.jdn.restdemo.model.Address;
import com.jdn.restdemo.model.Person;
import com.jdn.restdemo.repository.PersonRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.boot.CommandLineRunner;
import org.springframework.stereotype.Component;

@Component
@RequiredArgsConstructor
public class SampleDataLoader implements CommandLineRunner {
    private final PersonRepository repository;



    @Override
    public void run(String... args) throws Exception {
        Person person = new Person("Jan", "De Niels", "jdn@gmail.com", new Address("Street", "Citry"));
        repository.save(person);
    }
//    private final Faker faker;
}

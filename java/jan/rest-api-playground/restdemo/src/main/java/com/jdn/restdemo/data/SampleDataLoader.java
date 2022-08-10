package com.jdn.restdemo.data;

import com.github.javafaker.Faker;
import com.jdn.restdemo.model.Address;
import com.jdn.restdemo.model.Person;
import com.jdn.restdemo.repository.PersonRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.boot.CommandLineRunner;
import org.springframework.stereotype.Component;

import java.util.List;
import java.util.stream.IntStream;

@Component
@RequiredArgsConstructor
public class SampleDataLoader implements CommandLineRunner {
    private final PersonRepository repository;
    private final Faker faker;



    @Override
    public void run(String... args) throws Exception {
//        Person person = new Person("Jan", "De Niels", "jdn@gmail.com", new Address("Street", "Citry"));
//        repository.save(person);
        List<Person> people = IntStream.rangeClosed(1,100)
                .mapToObj(i -> new Person(
                        faker.name().firstName(),
                        faker.name().lastName(),
                        faker.internet().emailAddress(),
                        new Address(
                                faker.address().streetAddress(),
                                faker.address().city()
                        )
                )).toList();

        repository.saveAll(people);
    }
//    private final Faker faker;
}

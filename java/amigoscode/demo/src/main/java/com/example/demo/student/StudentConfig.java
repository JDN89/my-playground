package com.example.demo.student;

import org.springframework.boot.CommandLineRunner;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import java.time.LocalDate;
import java.time.Month;
import java.util.List;

@Configuration
public class StudentConfig {

    @Bean
    CommandLineRunner commandLineRunner(StudentRepository repository) {
        return args -> {
            Student jan = new Student(1L, "Jan", "jan@test.com", LocalDate.of(1999, Month.JANUARY , 5), 22);
            Student alex = new Student("Alex", "alex@test.com", LocalDate.of(1999, Month.APRIL, 5), 22);
            repository.saveAll(List.of(jan,alex));
        };
    }
}

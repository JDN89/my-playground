package com.example.demo.student;

import org.springframework.boot.CommandLineRunner;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import java.time.LocalDate;
import java.time.Month;
import java.util.List;

@Configuration
public class StudentConfig {
// REMINDER -> how does DB get dropped and recreated if student db exists?
    // read hibernate doc
    @Bean
    CommandLineRunner commandLineRunner(StudentRepository repository) {
        return args -> {
            Student jan = new Student(1L, "Jan", "jan@test.com", LocalDate.of(1999, Month.JANUARY , 5));
            Student alex = new Student("Alex", "alex@test.com", LocalDate.of(1999, Month.APRIL, 5) );
            repository.saveAll(List.of(jan,alex));
        };
    }
}

package com.jdn.restdemo.repository;

import com.jdn.restdemo.model.Person;
import org.springframework.data.repository.PagingAndSortingRepository;
import org.springframework.stereotype.Repository;

import java.util.Optional;

@Repository
public interface PersonRepository extends PagingAndSortingRepository<Person, Integer> {
    //JBQL
    //@Query("SELECT s FROM Student s WHERE s.email = ?1")
    Optional<Person> findPersonByEmail(String email);

    Optional<Person> findPersonById(Integer id);

}
